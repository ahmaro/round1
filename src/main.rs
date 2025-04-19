use HashTable::hash::HashTable;

fn main() {
    // PART II
    const GIVEN_TXT: &str = "https://www.gutenberg.org/files/98/98-0.txt";
    let given_book = DataFetcher::data_fetcher::get_body(&GIVEN_TXT);

    // println!("Body is:\n{}", given_book);

    let res = Keys::keys::pair_keys(&given_book);
    let pairs_num = res.clone().len() + 1;
    println!("The number of generated keys is: {}", pairs_num);

    let mut main_hashtable = HashTable::new(pairs_num-1);
    for (key, value) in res.clone() {
        // println!("The key to insert: {}, {}", key, value);
        main_hashtable.insert((key, value));
    }

    let the_first_pair: (String, usize) = main_hashtable.get_first();
    // println!("Get first pairs: {} ,  {}", the_first_pair.0, the_first_pair.1);

    let the_last_pair: (String, usize) = main_hashtable.get_last();
    // println!("Get last pairs: {} ,  {}", the_last_pair.0, the_last_pair.1);

    let key_to_remove_get: String = String::from("affidavit");
    match  main_hashtable.remove(key_to_remove_get.clone()) {
        Some(_) => println!("Remove is successful!"),
        None => println!("Remove:Key not found!"),
    };

    match  main_hashtable.remove(key_to_remove_get.clone()) {
        Some(_) => println!("Remove is successful!"),
        None => println!("Remove again: Key not found!"),
    };

    match  main_hashtable.get(key_to_remove_get.clone()) {
        Some(_) => println!("Get is successful!"),
        None => println!("Get: Key not found!"),
    };


    // PART III
    Trading::trading::retrieve_and_parse_tickers();
}
