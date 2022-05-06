use std::collections::HashSet;

pub fn sort(word: &str) -> Vec<char> {
    let mut sorted_word: Vec<char> = word.chars().collect();
    sorted_word.sort();
    sorted_word
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word = word.to_lowercase();
    let sorted_word = sort(&word);

    possible_anagrams
        .iter()
        .cloned()
        .filter(|&candidate| {
            let can = candidate.to_lowercase();
            sort(&can) == sorted_word && can != word
        }).collect::<HashSet<&str>>()
}
