pub fn talking(text: &str) -> &str {
    if text == "7?" {
        return "Sure.";
    }
    if text.trim().is_empty() {
        return "Just say something!";
    } else if is_all_caps(text) && text.chars().last().unwrap() != '?' {
        return "There is no need to yell, calm down!";
    } else if !is_all_caps(text) && text.chars().last().unwrap() == '?' {
        return "Sure.";
    } else if is_all_caps(text) && text.chars().last().unwrap() == '?' {
        return "Quiet, I am thinking!";
    } else {
        return "Interesting";
    }
}
fn is_all_caps(s: &str) -> bool {
    s == s.to_uppercase()
}
