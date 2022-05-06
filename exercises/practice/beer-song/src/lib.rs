const ZERO_VERSE: &str = "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n";
const ONE_VERSE: &str = "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n";
const TWO_VERSE: &str = "2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n";

pub fn verse(n: u32) -> String {
    let result = match n {
        0 => ZERO_VERSE.to_owned(),
        1 => ONE_VERSE.to_owned(),
        2 => TWO_VERSE.to_owned(),
        3.. => format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.\n", n, n, n-1),
    };

    result.to_owned()
}

pub fn sing(start: u32, end: u32) -> String {
    let mut result = String::from("");
    for i in (end..=start).rev() {
        result.push_str(&verse(i));
        if i != end {
            result.push('\n');
        }
    }
    result
}
