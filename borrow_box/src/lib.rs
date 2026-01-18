#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GameSession {
    pub id: u32,
    pub p1: (String, u32),
    pub p2: (String, u32),
    pub nb_games: u32,
}

impl GameSession {
    pub fn new(id: u32, p1_name: String, p2_name: String, nb_games: u32) -> GameSession {
        GameSession {
            id: id,
            p1: (p1_name, 0),
            p2: (p2_name, 0),
            nb_games: nb_games,
        }
    }

    pub fn read_winner(&self) -> Option<&(String, u32)> {
        if self.p1.1 > self.p2.1 {
            return Some(&self.p1);
        } else if self.p2.1 > self.p1.1 {
            return Some(&self.p2);
        } else {
            return None;
        }
    }

    pub fn update_score(&mut self, user_name: &str) {
        let win = self.nb_games / 2 + 1;
        let first = user_name == self.p1.0;
        let second = user_name == self.p2.0;
        if user_name == self.p1.0 || user_name == self.p2.0 {
            if self.p1.1 == win || self.p2.1 == win {
                return;
            } else {
                if first {
                    self.p1.1 += 1;
                }
                if second {
                    self.p2.1 += 1;
                }
            }
        }
    }

    pub fn delete(self) -> String {
        return format!("game deleted: id -> {}", self.id);
    }
}
