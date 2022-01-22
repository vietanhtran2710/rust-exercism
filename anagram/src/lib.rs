use std::collections::HashSet;
use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;

pub fn counter(word: &str) -> HashMap<String, u16> {
    let mut result: HashMap<String, u16> = HashMap::new();
    for ch in word.graphemes(true).into_iter() {
        let lower = &ch.to_lowercase();
        println!("{}", lower);
        if result.contains_key(lower) {
            *result.get_mut(lower).unwrap() += 1;
        }
        else {
            result.insert(lower.to_string(), 1);
        }
    }
    result
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut result: HashSet<&'a str> = HashSet::new();
    let word_counter = counter(&word);
    for item in possible_anagrams {
        if item.to_lowercase() != word.to_lowercase() {
            let item_counter = counter(&item);
            if word_counter == item_counter {
                result.insert(item);
            }
        }   
    }
    result
}
