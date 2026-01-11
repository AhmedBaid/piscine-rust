#[derive(Debug, PartialEq)]
pub struct CipherError {
    pub expected: String,
}

pub fn cipher(original: &str, ciphered: &str) -> Result<(), CipherError> {
    let mut str = String::new();
    for ch in original.chars() {
        if !ch.is_alphabetic() {
            str.push(ch);
            continue;
        }
        if ch.is_ascii_uppercase() {
            let ascii = ch as u8;
            let upper_value = 90 - (ascii - 65);
            str.push(upper_value as char);
        } else {
            let ascii = ch as u8;
            let lower_value = 122 - (ascii - 97);
            str.push(lower_value as char);
        }
    }
    if str == ciphered {
        return Ok(());
    } else {
        return Err(CipherError {
            expected: str.to_string(),
        });
    }
}
