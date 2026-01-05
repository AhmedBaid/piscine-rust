pub fn delete_and_backspace(s: &mut String) {
    let mut res: String = String::new();
    let mut delete_next: bool = false;
    let mut count: i32 = 0;
    for ch in s.chars() {
        if ch == '-' {
            res.pop();
            continue;
        }
        if ch == '+' {
            delete_next = true;
            count += 1;
            continue;
        }

        if delete_next == true || count > 0 {
            delete_next = false;
            count -= 1;
        } else {
            res.push(ch);
        }
    }

    *s = res;
}

pub fn do_operations(v: &mut [String]) {
    let mut res: Vec<String> = Vec::new();
    for ele in v {
        let plus = ele.find('+').unwrap_or(0);
        let moins = ele.find('-').unwrap_or(0);
        if plus > 0 {
            let (first, last) = ele.split_at(plus);
            let nb1: i32 = first.parse().unwrap();
            let nb2: i32 = last.parse().unwrap();
            *ele= (nb1 + nb2).to_string();
        } else {
            let (first, last) = ele.split_at(moins);
            let nb1: i32 = first.parse().unwrap();
            let nb2: i32 = last.parse().unwrap();
            *ele= (nb1 - nb2).to_string();
        }
    }
}
