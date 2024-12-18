use lapin::{
    options::*, types::FieldTable, BasicProperties, Connection, ConnectionProperties,
};
use rand::Rng;
use serde::Serialize;
use tokio::time::{sleep, Duration};

#[derive(Debug, Serialize)] // Added Debug trait
struct StockMessage {
    stock: String,
    price: f64,
}

#[tokio::main]
async fn main() {
    // Connect to RabbitMQ
    let conn = Connection::connect("amqp://127.0.0.1:5672/%2f", ConnectionProperties::default())
        .await
        .expect("Failed to connect to RabbitMQ");

    let channel = conn.create_channel().await.expect("Failed to create channel");

    // Declare the queue
    channel
        .queue_declare(
            "stock_prices",
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await
        .expect("Queue declaration failed");

    let stocks = vec!["AAPL", "MSFT", "TSLA", "GOOG"];
    println!("Publishing stock prices...");

    let mut rng = rand::thread_rng();

    loop {
        let stock = stocks[rng.gen_range(0..stocks.len())];
        let price = rng.gen_range(80.0..200.0);

        let message = StockMessage {
            stock: stock.to_string(),
            price,
        };

        let payload = serde_json::to_string(&message).unwrap();

        channel
            .basic_publish(
                "",                  // Default exchange
                "stock_prices",      // Queue name
                BasicPublishOptions::default(),
                payload.as_bytes(),  // Message payload
                BasicProperties::default(), // Updated usage of BasicProperties
            )
            .await
            .expect("Failed to publish message");

        println!("Sent: {:?}", message);
        sleep(Duration::from_secs(1)).await; // Delay to simulate real-time updates
    }
}
