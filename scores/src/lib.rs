use std::collections::HashMap;

pub fn score(str: &str) -> u64 {
    let mut map = HashMap::new();
    let mut count = 0;
    map.insert("aeioulnrst", 1);
    map.insert("dg", 2);
    map.insert("bcmp", 3);
    map.insert("fhvwy", 4);
    map.insert("k", 5);
    map.insert("jx", 8);
    map.insert("qz", 10);

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
