pub fn is_pangram(s: &str) -> bool {
    let str = s.to_lowercase();
    let all = "abcdefghijklmnopqrstuvwxyz";
    for ch in all.chars() {
        if !str.contains(ch) {
            return false;
        }
    }
    true
}
