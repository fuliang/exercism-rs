// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

fn word_count<'a> (words: &[&'a str], count_map: &mut HashMap<&'a str, u32>) {
    for word in words {
        *count_map.entry(*word).or_insert(0) += 1;
    }
}

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut magazine_count_map: HashMap<&str, u32> = HashMap::new();
    word_count(magazine, &mut magazine_count_map);

    let mut note_count_map: HashMap<&str, u32> = HashMap::new();
    word_count(note, &mut note_count_map);
    
    for (word, count) in note_count_map {
        match magazine_count_map.get(word) {
            Some(&m_count) =>  {
                if m_count < count {
                    return false;
                }
            },
            _ => return false,
        }
    }
    true
}


#[test]
fn test_word_count() {
    let words = ["hello", "world", "hello", "world"];

    let mut count_map: HashMap<&str, u32> = HashMap::new();
    word_count(&words, &mut count_map);
    assert_eq!(count_map.get("hello"), Some(&2));
    assert_eq!(count_map.get("world"), Some(&2));
    assert_eq!(count_map.get("nothing"), None);
}