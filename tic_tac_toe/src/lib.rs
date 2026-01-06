pub fn tic_tac_toe(table: [[char; 3]; 3]) -> String {
    let xo = "XO";
    for ch in xo.chars() {
        if diagonals(ch, table) {
            return format!("player {} won", ch);
        } else if horizontal(ch, table) {
            return format!("player {} won", ch);
        } else if vertical(ch, table) {
            return format!("player {} won", ch);
        }
    }
    return format!("tie");
}

pub fn diagonals(player: char, table: [[char; 3]; 3]) -> bool {
    if (table[0][0] == player && table[1][1] == player && table[2][2] == player)
        || (table[0][2] == player && table[1][1] == player && table[2][0] == player)
    {
        return true;
    }
    return false;
}

pub fn horizontal(player: char, table: [[char; 3]; 3]) -> bool {
    let rows = table.len();
    for row in 0..rows {
        if table[row][0] == player && table[row][1] == player && table[row][2] == player {
            return true;
        }
    }
    return false;
}

pub fn vertical(player: char, table: [[char; 3]; 3]) -> bool {
    let rows = table.len();
    let cols = table[0].len();
    let mut count = 0;
    for row in 0..rows {
        for col in 0..cols {
            if table[col][row] == player {
                count += 1
            }
        }
        if count == 3 {
            return true;
        }
        count = 0
    }
    return false;
}
