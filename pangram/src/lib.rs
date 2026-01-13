pub fn is_pangram(s: &str) -> bool {
    let all = "abcdefghijklmnopqrstuvwxyz";
    for ch in all.chars(){
        if !s.contains(ch){
            return false
        }
    } 
    true
}