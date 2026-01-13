use std::collections::HashMap;

pub fn score(str: &str) -> u64 {
    let mut map = HashMap::new();
    let mut count = 0;
    map.insert("aeioulnrst", 1);
    map.insert("dg", 1);
    map.insert("bdmp", 1);
    map.insert("fhvmy", 1);
    map.insert("k", 1);
    map.insert("js", 1);
    map.insert("qz", 1);

    for ch in str.chars() {
        for (key, value) in &map {
            if key.contains(ch.to_ascii_lowercase()) {
                count += value;
            }
        }
    }
    count
}
