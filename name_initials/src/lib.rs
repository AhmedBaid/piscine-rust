pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();
    let mut f = String::new();
    for element in names.iter() {
        f.push_str(&element[0..1]);
        f.push('.');
        f.push(' ');
        for (index, rune) in element.chars().enumerate(){
            if rune == ' '{
                f.push_str(&element[index + 1..index + 2]);
                f.push('.');
            }
        }
        res.push(f.clone());
        f.clear();
    }
    res
}
