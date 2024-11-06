
pub fn get_unique_letters(word: &str) -> Vec<char> {
    let mut present_chars: Vec<char> = Vec::with_capacity(word.len());
    for char in word.chars() {
        if !present_chars.contains(&char) {
            present_chars.push(char);
        }
    }
    present_chars
}

pub fn get_point_value(word: &str) -> u64{
    word.len() as u64
}