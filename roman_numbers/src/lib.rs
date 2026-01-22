use std::collections::BTreeMap;

use crate::RomanDigit::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RomanDigit {
    Nulla,
    I,
    V,
    X,
    L,
    C,
    D,
    M,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RomanNumber(pub Vec<RomanDigit>);

impl From<u32> for RomanNumber {
    fn from(value: u32) -> Self {
        match value {
            0 => RomanNumber(vec![Nulla]),
            _ => {
                let mut num = value;
                let mut digits = Vec::new();
                let roman_nums = BTreeMap::from([
                    (1000, [M].to_vec()),
                    (900, [C, M].to_vec()),
                    (500, [D].to_vec()),
                    (400, [C, D].to_vec()),
                    (100, [C].to_vec()),
                    (90, [X, C].to_vec()),
                    (50, [L].to_vec()),
                    (40, [X, L].to_vec()),
                    (10, [X].to_vec()),
                    (9, [I, X].to_vec()),
                    (5, [V].to_vec()),
                    (4, [I, V].to_vec()),
                    (1, [I].to_vec()),
                ]);

                for (val, digit) in roman_nums.iter().rev() {
                    while num >= *val {
                        for d in digit {
                            digits.push(*d);
                        }
                        num -= *val;
                    }
                }

                RomanNumber(digits)
            }
        }
    }
}
