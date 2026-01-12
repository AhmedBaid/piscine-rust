use convert_case::{Case, Casing};

pub fn expected_variable(compared: &str, expected: &str) -> Option<String> {
    let compared_lower = compared.to_lowercase();
    let expected_lower = expected.to_lowercase();

    if !compared_lower.is_case(Case::Camel) && !expected_lower.is_case(Case::Snake) {
        return None;
    }

    let distance = edit_distance(&compared_lower, &expected_lower);

    let porc = 100 - (distance as isize * 100 / expected.len() as isize);
    if porc > 50 {
        Some(format!("{}%", porc))
    } else {
        None
    }
}

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

    for i in 1..=len_src {
        for j in 1..=len_targ {
            let cost = if src[i - 1] == targ[j - 1] { 0 } else { 1 };
            let insert = matrix[i - 1][j] + 1;
            let delate = matrix[i][j - 1] + 1;
            let replace = matrix[i - 1][j - 1] + cost;
            matrix[i][j] = std::cmp::min(std::cmp::min(insert, delate), replace);
        }
    }

    matrix[len_src][len_targ]
}
