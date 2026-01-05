pub fn to_url(s: &str) -> String {
    let result = s.replace(" ", "%20");
    result
}
