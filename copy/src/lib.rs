pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    (c, (c as f64).exp(), (c.abs() as f64).ln())
}

pub fn str_function(a: String) -> (String, String) {
    let mut res = String::new();
    for rune in a.chars() {
        if rune == ' ' {
            res.push(rune)
        } else {
            let value = (rune.to_digit(10).unwrap_or(0) as f64).exp();
            res.push_str(&value.to_string());
        }
    }
    (a, res)
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let mut res: Vec<f64> = Vec::new();
    for ele in &b {
        let f: f64 = ((*ele).abs() as f64).ln();
        res.push(f);
    }
    (b, res)
}
