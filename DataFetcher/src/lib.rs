pub mod data_fetcher { 
    pub fn get_body(this_url: &str) -> String {
        loop {
            let text = reqwest::blocking::get(this_url).unwrap().text();
            match text {
                Ok(body) => return String::from(body),
                Err(_) => {
                    println!("Failed! Trying again...");
                    continue;
                },
            }
        }
         
    }
}