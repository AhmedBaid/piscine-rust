pub fn capitalize_first(input: &str) -> String {
    let mut res = String::new();
    for (i, ch) in input.chars().enumerate() {
        if i == 0 {
            let upper: String = ch.to_uppercase().collect();
            res.push_str(&upper);
        } else {
            res.push(ch);
        }
    }
    res
}

pub fn title_case(input: &str) -> String {
    let mut res = String::new();
    let mut new_word = true;

    for ch in input.chars() {
        if ch.is_whitespace() {
            res.push(ch);
            new_word = true;
        } else {
            if new_word {
                res.push(ch.to_ascii_uppercase());
                new_word = false;
            } else {
                res.push(ch.to_ascii_lowercase());
            }
        }
    }

    res
}

pub fn change_case(input: &str) -> String {
    let mut res = String::new();
    for ch in input.chars() {
        if ch.is_lowercase() {
            res.push(ch.to_ascii_uppercase())
        } else {
            res.push(ch.to_ascii_lowercase())
        }
    }
    res
}
