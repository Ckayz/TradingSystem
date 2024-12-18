use futures_util::StreamExt; // Import StreamExt for next()
use lapin::{options::*, types::FieldTable, Connection, ConnectionProperties};
use serde::Deserialize;
use std::{collections::HashMap, sync::mpsc, thread};

#[derive(Debug, Deserialize)]
struct StockMessage {
    stock: String,
    price: f64,
}

// Decision engine to process stock prices
fn decision_engine(receiver: std::sync::mpsc::Receiver<StockMessage>) {
    let mut owned_stocks: HashMap<String, usize> = HashMap::new();
    let buy_threshold = 100.0;
    let sell_threshold = 150.0;

    while let Ok(stock) = receiver.recv() {
        if stock.price < buy_threshold {
            println!("BUY ORDER: {} at ${}", stock.stock, stock.price);
            *owned_stocks.entry(stock.stock.clone()).or_insert(0) += 1;
        } else if stock.price > sell_threshold {
            if let Some(count) = owned_stocks.get_mut(&stock.stock) {
                if *count > 0 {
                    println!("SELL ORDER: {} at ${}", stock.stock, stock.price);
                    *count -= 1;
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    // Connect to RabbitMQ
    let conn = Connection::connect("amqp://127.0.0.1:5672/%2f", ConnectionProperties::default())
        .await
        .expect("Failed to connect to RabbitMQ");

    let channel = conn.create_channel().await.unwrap();

    // Declare the stock_prices queue
    channel
        .queue_declare(
            "stock_prices",
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await
        .unwrap();

    println!("Waiting for stock prices...");

    let (tx, rx) = mpsc::channel();

    // Spawn the decision engine in a separate thread
    thread::spawn(move || decision_engine(rx));

    let mut consumer = channel
        .basic_consume(
            "stock_prices",
            "trading_consumer",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await
        .unwrap();

    // Use StreamExt::next() to process messages
    while let Some(delivery_result) = consumer.next().await {
        match delivery_result {
            Ok(delivery) => {
                let message: StockMessage =
                    serde_json::from_slice(&delivery.data).expect("Failed to parse message");
                println!("Received: {:?}", message);
                tx.send(message).expect("Failed to send message to decision engine");
            }
            Err(err) => {
                eprintln!("Error receiving message: {:?}", err);
            }
        }
    }
}
