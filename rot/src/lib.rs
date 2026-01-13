pub fn rotate(input: &str, key: i8) -> String {
    let mut result = String::new();

    let shift = ((key % 26) + 26) % 26;

    for ch in input.chars() {
        if ch.is_ascii_lowercase() {
            let rotated = (ch as u8 - 97 + shift as u8) % 26 + 97;
            result.push(rotated as char);
        } else if ch.is_ascii_uppercase() {
            let rotated = (ch as u8 - 65 + shift as u8) % 26 + 65;
            result.push(rotated as char);
        } else {
            result.push(ch);
        }
    }

    result
}
