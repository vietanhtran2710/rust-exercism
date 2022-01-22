// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut count: HashMap<&str, u32> = HashMap::new();
    for word in magazine {
        if count.contains_key(word) {
            *count.get_mut(word).unwrap() += 1;
        }
        else {
            count.insert(word, 1);
        }
    }
    for word in note {
        if !count.contains_key(word) {
            return false;
        }
        else {
            *count.get_mut(word).unwrap() -= 1;
            if *count.get(word).unwrap() == 0 {
                count.remove(word);
            }
        }
    }
    return true;
}
