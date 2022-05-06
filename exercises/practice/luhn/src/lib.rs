/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {

    let chars: Vec<char> = code.chars().filter(|ch| !ch.is_whitespace()).collect();
    if chars.len() <= 1 || chars.iter().any(|ch| !ch.is_digit(10)) {
        return false;
    }

    chars
    .iter()
    .rev()
    .map(|ch| ch.to_digit(10).unwrap())
    .enumerate()
    .map(|(i, ch)| if i % 2 == 0 || ch == 9 { ch } else { ch * 2 % 9})
    .sum::<u32>() % 10 == 0
}

#[test]
fn test_num() {
    let m = is_valid("059");
    println!("{:?}", m);
}