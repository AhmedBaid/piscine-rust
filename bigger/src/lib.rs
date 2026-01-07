use std::collections::HashMap;

pub fn bigger(h: HashMap<&str, i32>) -> i32 {
    let mut res = Vec::new();
    for (_, value) in h {
        res.push(value);
    }
    *res.iter().max().unwrap()
}
