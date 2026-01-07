use std::collections::HashMap;

pub fn word_frequency_counter<'a>(words: &[&'a str]) -> HashMap<&'a str, usize> {
    let mut data: HashMap<&str, usize> = HashMap::new();
    for ele in words {
        if !data.contains_key(ele) {
            println!("words : {}", ele);
            data.insert(ele, 1);
        } else {
            println!("{}", ele);
            data.insert(ele, data.get(ele).unwrap() + 1);
        }
    }
    data
}

pub fn nb_distinct_words(frequency_count: &HashMap<&str, usize>) -> usize {
    frequency_count.len()
}
