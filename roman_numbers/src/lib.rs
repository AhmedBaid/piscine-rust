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

                let roman_numerals = [
                    (1000, M),
                    (900, C),
                    (500, D),
                    (400, C),
                    (100, C),
                    (90, X),
                    (50, L),
                    (40, X),
                    (10, X),
                    (9, I),
                    (5, V),
                    (4, I),
                    (1, I),
                ];

                for &(val, digit) in roman_numerals.iter() {
                    while num >= val {
                        digits.push(digit);
                        num -= val;
                    }
                }

                RomanNumber(digits)
            }
        }
    }
}
