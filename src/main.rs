// Part I
/* This is actually my first project with rust. I have known Rust from articles and 
   former colleages who started recently switching to Rust, therefore that pushed me to 
   learn more about it. I have encountered many of the concepts from my C++ experience
   in Autosar that are being used on a regular basis like Result, Option... I used couple
   of times match, impl, struct, enum, mut and few of times, .clone, .unwrap. */

/* I used cargo to manage the packages seamlessly though pachages and created libs. */
/* I would give myself a rate of 5.5 as there are certainly more advanced topics that still
   I did not explore with rust! Nevertheless, I am quite confident to work in depth with 
   Rust. If given the opportunity, I’ll ramp up quickly, contribute meaningfully, and
   keep leveling up my Rust expertise. */

/* For clarity, I created four seperate libraries to get the best effective approach to 
   perform a working functional solution. They are: DataFetscher, HashTable, Keys, Trading. */

use HashTable::hash::HashTable;

fn main() {
    // PART II
    // Fetch the book text
    const GIVEN_TXT: &str = "https://www.gutenberg.org/files/98/98-0.txt";
    let given_book = DataFetcher::data_fetcher::get_body(&GIVEN_TXT);

    // println!("Body is:\n{}", given_book);

    // Generate paired keys out of the book. Assumed values are the words' repeatances.
    let res = Keys::keys::pair_keys(&given_book);
    let pairs_num = res.clone().len() + 1;
    println!("The number of generated keys is: {}", pairs_num);

    // Implement a fixed sized open addressing hash table by using linear probing.
    let mut main_hashtable = HashTable::new(pairs_num-1);
    for (key, value) in res.clone() {
        // println!("The key to insert: {}, {}", key, value);
        main_hashtable.insert((key, value));
    }

    // Test the required APIs:
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
    // Retrieve the tickers and parse them.
    Trading::trading::retrieve_and_parse_tickers();
}
