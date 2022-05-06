pub fn abbreviate(phrase: &str) -> String {
    if phrase.is_empty() {
        return String::from("");
    }

    let mut result = String::from("");

    for s in phrase.split(" ") {
        if s.chars().all(|c| c.is_uppercase()) {
            result.push(s.chars().next().unwrap());
        } else {
            s.chars().enumerate()
            .filter(|(index, e)| *index == 0 && e.is_alphabetic() || e.is_uppercase())
            .map(|(_, e)| e.to_ascii_uppercase())
            .for_each(|ch| result.push(ch));
        }
    }
    result
}
