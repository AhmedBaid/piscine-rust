use std::fmt;

#[derive(Debug)]
pub struct Player<'a> {
    pub name: &'a str,
    pub strength: f64,
    pub score: u32,
    pub money: u32,
    pub weapons: Vec<&'a str>,
}

pub struct Fruit {
    pub weight_in_kg: f64,
}

pub struct Meat {
    pub weight_in_kg: f64,
    pub fat_content: f64,
}

impl Player<'_> {
    pub fn eat(&mut self, food: impl Food) {
        self.strength += food.gives();
    }
}

impl fmt::Display for Player<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let strength_int = self.strength.trunc() as i64;

        writeln!(f, "{}", self.name)?;
        writeln!(
            f,
            "Strength: {}, Score: {}, Money: {}",
            strength_int, self.score, self.money
        )?;
        write!(f, "Weapons: {:?}", self.weapons)
    }
}

pub trait Food {
    fn gives(&self) -> f64;
}

impl Food for Fruit {
    fn gives(&self) -> f64 {
        self.weight_in_kg * 4.0
    }
}

impl Food for Meat {
    fn gives(&self) -> f64 {
        let fat_kg = self.weight_in_kg * self.fat_content;
        let protein_kg = self.weight_in_kg - fat_kg;
        protein_kg * 4.0 + fat_kg * 9.0
    }
}
