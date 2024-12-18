use std::collections::HashMap;
use std::thread;
use std::time::Duration;
use rand::Rng;

// Structure to represent stock data
struct StockMessage {
    stock: String,
    price: f64,
}

fn main() {
    let stocks = vec!["AAPL", "MSFT", "TSLA", "GOOG", "AMZN"]; // List of stocks
    let buy_threshold = 100.0;  // Price to trigger a buy
    let sell_threshold = 150.0; // Price to trigger a sell

    // Track owned stocks and inventory
    let mut inventory: HashMap<String, i32> = HashMap::new();

    println!("--- Real-Time Stock Trading System Simulation ---\n");

    loop {
        // Simulate price updates for each stock
        for stock in &stocks {
            let price = simulate_stock_price();

            // Print received stock price
            println!("📈 Stock: {:<5} | Price: ${:<6.2}", stock, price);

            // Decision: Buy or Sell
            if price < buy_threshold {
                println!("💰 BUY ORDER: {:<5} at ${:<6.2}", stock, price);
                *inventory.entry(stock.to_string()).or_insert(0) += 1;
            } else if price > sell_threshold {
                if let Some(count) = inventory.get_mut(*stock) {
                    if *count > 0 {
                        println!("💸 SELL ORDER: {:<5} at ${:<6.2}", stock, price);
                        *count -= 1;
                    }
                }
            }

            // Print current inventory
            println!("📊 Inventory: {:?}\n", inventory);

            // Sleep for a short duration to simulate real-time updates
            thread::sleep(Duration::from_millis(500));
        }
    }
}

// Function to simulate random stock price generation
fn simulate_stock_price() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(80.0..200.0) // Random price between 80 and 200
}
