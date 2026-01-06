pub fn edit_distance(source: &str, target: &str) -> usize {
    let src: Vec<char> = source.chars().collect();
    let targ: Vec<char> = target.chars().collect();

    let len_src = src.len();
    let len_targ = targ.len();

    let mut matrix = vec![vec![0; len_targ + 1]; len_src + 1];

    for i in 0..=len_src {
        matrix[i][0] = i;
    }
    for j in 0..=len_targ {
        matrix[0][j] = j;
    }
    println!("{:?}",matrix);

    for i in 1..=len_src {
        for j in 1..=len_targ {
            let cost = if src[i - 1] == targ[j - 1] { 0 } else { 1 };

            matrix[i][j] = std::cmp::min(
                std::cmp::min(matrix[i - 1][j] + 1, matrix[i][j - 1] + 1),
                matrix[i - 1][j - 1] + cost,
            );
        }
    }

    matrix[len_src][len_targ]
}
