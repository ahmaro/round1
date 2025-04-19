pub mod trading {
    use serde_json::from_str;
    use serde::{Deserialize, Serialize};
    use std::process::Command;
    use std::time::Instant;

    // Struct to put out the a single ticker info.
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct OneTricker {
        symbol: String,
        price_change: String,
        price_change_percent: String,
        last_price: String,
        last_qty: String,
        open: String,
        high: String,
        low: String,
        volume: String,
        amount: String,
        bid_price: String,
        ask_price: String,
        open_time: i64,
        close_time: i64,
        first_trade_id: i64,
        trade_count: i64,
        strike_price: String,
        exercise_price: String,
    }

    // Perform fetch ticker from url and parse them.
    pub fn retrieve_and_parse_tickers() {
        // use this time linux environment.
        let output = Command::new("curl")
            .arg("-s") 
            .arg("-X")
            .arg("GET")
            .arg("https://eapi.binance.com/eapi/v1/ticker")
            .output()
            .expect("Failed to execute curl command");

        if output.status.success() {
            let result = String::from_utf8_lossy(&output.stdout);
            // Parsing via serde
            let start = Instant::now();
            let price_data: Result<Vec<OneTricker>, serde_json::Error> = from_str(&result);
            match price_data {
                Ok(tickers) => {
                    let duration = start.elapsed();
                    println!("Let us print the results!!!! Just the symbol and its price:");
                    for single_ticker in tickers {
                        println!("The symbol is {} and its price {}", single_ticker.symbol, single_ticker.last_price);
                    }
                    println!("Parsing a ticker duration ís: {:?}", duration);
                },
                Err(_) => {
                    eprintln!("Error parsing tickers.");
                },
            }
        } else {
            eprintln!("Error in the linux command.");
            /* Optimizations I would consider:
                - That I would only parse the objects or the info that are needed or required instead of 17 entries are given by each ticker.
                - Use my own parser, like using text splitting or filters as were used in parsing the book.
                - Optimize the compiler flags.
                - Check stack memory usage.
             */
        }
    }
}