use std::{fmt, str::FromStr};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Antigen {
    A,
    AB,
    B,
    O,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum RhFactor {
    Positive,
    Negative,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct BloodType {
    pub antigen: Antigen,
    pub rh_factor: RhFactor,
}

impl FromStr for BloodType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (antigen_str, rh_str) = s.split_at(s.len() - 1);
        let antigen = match antigen_str {
            "A" => Antigen::A,
            "B" => Antigen::B,
            "AB" => Antigen::AB,
            "O" => Antigen::O,
            _ => return Err(()),
        };
        let rh_factor = match rh_str {
            "+" => RhFactor::Positive,
            "-" => RhFactor::Negative,
            _ => return Err(()),
        };
        Ok(BloodType { antigen, rh_factor })
    }
}

impl fmt::Debug for BloodType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let antigen_str = match self.antigen {
            Antigen::A => "A",
            Antigen::B => "B",
            Antigen::AB => "AB",
            Antigen::O => "O",
        };
        let rh_str = match self.rh_factor {
            RhFactor::Positive => "+",
            RhFactor::Negative => "-",
        };
        write!(f, "{}{}", antigen_str, rh_str)
    }
}

impl BloodType {
    pub fn can_receive_from(self, other: Self) -> bool {
        let can_antig = match self.antigen {
            Antigen::A => other.antigen == Antigen::A || other.antigen == Antigen::O,
            Antigen::B => other.antigen == Antigen::B || other.antigen == Antigen::O,
            Antigen::AB => true,
            Antigen::O => other.antigen == Antigen::O,
        };
        let can_rhfactor = match self.rh_factor {
            RhFactor::Positive => true,
            RhFactor::Negative => other.rh_factor == RhFactor::Negative,
        };
        can_antig && can_rhfactor
    }

    pub fn donors(self) -> Vec<Self> {
        get_all_bloods()
            .into_iter()
            .filter(|blood| self.can_receive_from(*blood))
            .collect()
    }

    pub fn recipients(self) -> Vec<Self> {
        get_all_bloods()
            .into_iter()
            .filter(|blood| blood.can_receive_from(self))
            .collect()
    }
}

fn get_all_bloods() -> Vec<BloodType> {
    vec![
        BloodType {
            antigen: Antigen::A,
            rh_factor: RhFactor::Positive,
        },
        BloodType {
            antigen: Antigen::O,
            rh_factor: RhFactor::Positive,
        },
        BloodType {
            antigen: Antigen::B,
            rh_factor: RhFactor::Positive,
        },
        BloodType {
            antigen: Antigen::AB,
            rh_factor: RhFactor::Positive,
        },
        BloodType {
            antigen: Antigen::A,
            rh_factor: RhFactor::Negative,
        },
        BloodType {
            antigen: Antigen::O,
            rh_factor: RhFactor::Negative,
        },
        BloodType {
            antigen: Antigen::B,
            rh_factor: RhFactor::Negative,
        },
        BloodType {
            antigen: Antigen::AB,
            rh_factor: RhFactor::Negative,
        },
    ]
}
