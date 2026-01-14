pub fn scytale_cipher(message: &str, i: usize) -> String {
    if i == 0 {
        return "".to_string();
    }
    let size = message.len() as f64 / i as f64;
    let mut spaces = 0;
    if size.fract() != 0.0 {
        let modulo = message.len() % i;
        spaces = i - modulo;
    }
    let mut word = message.to_string();
    for _ in 0..spaces {
        word.push(' ')
    }
    let new_size = word.len() / i;

    let mut res = String::from("");
    let mut matrix: Vec<Vec<char>> = Vec::new();
    let mut jj = i.clone();
    let mut kk = 0;
    for _ in 0..new_size {
        let mut arr: Vec<char> = word[kk..jj].chars().collect();
        jj += i;
        kk += i;
        matrix.push(arr.clone());
        arr.clear();
    }
    let mat_len = matrix.len();
    for z in 0..i {
        for y in 0..mat_len {
            res.push(matrix[y][z]);
        }
    }
    res.trim().to_string()
}
