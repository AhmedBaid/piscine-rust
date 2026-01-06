pub fn first_subword(s: String) -> String {
    let mut res = String::new();
    for (index, rune) in s.chars().enumerate() {
        if index > 0 && (rune.is_uppercase() || rune == '_') {
            break;
        }else{
            res.push(rune)
        }
    }
    res
}
