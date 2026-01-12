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
    let len_source = source.chars().count();
    let len_target = target.chars().count();

    let mut table = vec![vec![0; len_target + 1]; len_source + 1];

    for i in 0..=len_source {
        table[i][0] = i;
    }
    for i in 0..=len_target {
        table[0][i] = i;
    }

    let source_chars: Vec<char> = source.chars().collect();
    let target_chars: Vec<char> = target.chars().collect();

    for i in 1..=len_source {
        for j in 1..=len_target {
            if source_chars[i - 1] == target_chars[j - 1] {
                table[i][j] = table[i - 1][j - 1];
            } else {
                table[i][j] = 1 + table[i - 1][j - 1].min(table[i][j - 1].min(table[i - 1][j]));
            }
        }
    }
    table[len_source][len_target]
}
