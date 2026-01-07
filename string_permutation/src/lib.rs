pub fn is_permutation(s1: &str, s2: &str) -> bool {
    let mut sort1: Vec<char> = s1.chars().collect();
    sort1.sort();
    let mut sort2: Vec<char> = s2.chars().collect();
    sort2.sort();
    let news1: String = sort1.iter().collect();
    let news2: String = sort2.iter().collect();
    news1.contains(&news2)
}
