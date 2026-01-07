use std::collections::HashMap;

pub fn mean(list: &[i32]) -> f64 {
    let mut count: i32 = 0;
    for ele in list {
        count += *ele;
    }
    count as f64 / list.len() as f64
}

pub fn median(list: &[i32]) -> i32 {
    let mut arr = Vec::new();
    for ele in list {
        arr.push(ele);
    }
    arr.sort();
    if arr.len() % 2 == 0 {
        return arr[arr.len() / 2 - 1] + arr[arr.len() / 2];
    } else {
        return *arr[arr.len() / 2];
    }
}

pub fn mode(list: &[i32]) -> i32 {
    let mut map = HashMap::new();
    for &value in list {
        *map.entry(value).or_insert(0) += 1;
    }

    map.into_iter()
        .max_by_key(|&(_,value)| value).map(|(key,_)| key).unwrap()
}
