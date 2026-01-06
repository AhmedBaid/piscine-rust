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
    let sl: Vec<String> = input.split_whitespace().map(|s| s.to_string()).collect();
    let res: Vec<String> = sl.iter().map(|ele| capitalize_first(ele)).collect();
    res.join(" ")
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
