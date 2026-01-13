use std::collections::HashMap;

pub fn score(str: &str) -> u64 {
    let mut map = HashMap::new();
    let mut count = 0;
    map.insert("aeioulnrst".to_string(), 1);
    map.insert("dg".to_string(), 2);
    map.insert("bcmp".to_string(), 3);
    map.insert("fhvwy".to_string(), 4);
    map.insert("k".to_string(), 5);
    map.insert("jx".to_string(), 8);
    map.insert("qz".to_string(), 10);

    let word = str.to_lowercase();
    for ch in word.chars() {
        for (key, value) in &map {
            if key.contains(ch) {
                count += value;
                break;
            }
        }
    }
    count
}
