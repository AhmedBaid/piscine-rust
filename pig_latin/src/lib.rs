pub fn pig_latin(text: &str) -> String {
    let arr: Vec<char> = text.chars().collect();
    let vowels = "aeiou";

    if vowels.contains(arr[0]) {
        return format!("{}ay", text);
    }
    if  arr[1] == 'q' && arr[2] == 'u' && !vowels.contains(arr[0]) {
        let rest: String = arr[2..].iter().collect();
        return  format!("{}quay", rest);
    }
    for (index, _) in arr.iter().enumerate() {
        if index != 0 && vowels.contains(arr[index]) {
            let first: String = arr[index..].iter().collect();
            let last: String = arr[..index].iter().collect();
            return  format!("{}{}ay", first, last);
        }
    }
    format!("{}", text)
}
