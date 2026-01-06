use std::collections::HashMap;

pub fn arrange_phrase(phrase: &str) -> String {
    let mut data = HashMap::new();
    let mut arr = Vec::new();
    let mut res = String::new();
    for word in phrase.split_whitespace() {
        let digit: String = word.chars().filter(|c| c.is_digit(10)).collect();
        let nb: u32 = digit.parse().expect("Not a valid number!");
        let filtered_string: String = word.chars().filter(|c: &char| !c.is_numeric()).collect();
        data.insert(nb, filtered_string);
    }
    for (key, _) in data.iter() {
        arr.push(key);
    }
    arr.sort();
    for i in arr {
        res.push_str(data.get(&i).unwrap());
        res.push(' ');
    }
    res.pop();
    res
}
