pub mod keys {
    use std::collections::HashMap;

    fn clean_words(text: &str) -> Vec<&str> {
        // not classified chars
        let to_remove_1 = '“';
        let to_remove_2 = '”';
        text.split(|c: char| c.is_whitespace() || c.is_ascii_punctuation() || c.eq(&to_remove_1) || c.eq(&to_remove_2))
            .filter(|word| !word.is_empty()).collect()
    }

    pub fn pair_keys(text: &str) -> HashMap<String, usize> {
        let mut map = HashMap::new();
        
        let cleaned = clean_words(text);
        for each_word in cleaned {
            let lowercased = each_word.to_lowercase();
            *map.entry(lowercased.to_string()).or_insert(1) += 1;
        }        
        map
    }
}