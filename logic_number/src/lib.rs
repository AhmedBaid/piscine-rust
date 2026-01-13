pub fn number_logic(num: u32) -> bool {
    let mut res = 0;
    let str = num.to_string();
    for ch in str.chars() {
        res += ch.to_digit(32).unwrap().pow(str.len() as u32)
    }
    res == num
}
