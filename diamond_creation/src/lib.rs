pub fn get_diamond(c: char) -> Vec<String> {
    if c == 'A' {
        return vec!["A".to_string()];
    }
    let mut res = Vec::new();
    if c.is_lowercase() {
        return vec![];
    }
    let ascii = c as u8 - 65;

    let mut charr = 65 as u8;
    let mut space_middle: usize = 0;
    for i in (0..=ascii).rev() {
        if i == ascii {
            let space1 = i;
            let space2 = i;
            let ele = format!(
                "{}A{}",
                " ".repeat(space1 as usize),
                " ".repeat(space2 as usize)
            );
            res.push(ele);
        } else if i == 0 {
            let ele = format!(
                "{}{}{}",
                c as char,
                " ".repeat(space_middle * 2 -1),
                c as char
            );
            res.push(ele);
        } else {
            let space1 = i;
            let space2 = i;
            let ele = format!(
                "{}{}{}{}{}",
                " ".repeat(space1 as usize),
                charr as char,
                " ".repeat(space_middle * 2 -1),
                charr as char,
                " ".repeat(space2 as usize)
            );
            res.push(ele);
        }
        space_middle += 1;
        charr += 1;
    }
    for index in (0..res.len() - 1).clone().rev() {
        res.push(res[index].to_string());
    }
    res
}