use rand::Rng;

#[derive(Debug,PartialEq)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}
#[derive(Debug,PartialEq)]
pub enum Rank {
    Ace,
    King,
    Queen,
    Jack,
    Number(u8),
}
impl Suit {
    pub fn random() -> Suit {
        let mut rng = rand::rng();
        let value: u8 = rng.random_range(1..=4);
        Suit::translate(value)
    }

    pub fn translate(value: u8) -> Suit {
        if value == 1 {
            Suit::Heart
        } else if value == 2 {
            Suit::Diamond
        } else if value == 3 {
            Suit::Spade
        } else {
            Suit::Club
        }
    }
}
impl Rank {
    pub fn random() -> Rank {
        let mut rng = rand::rng();
        let value: u8 = rng.random_range(1..=13);
        Rank::translate(value)
    }

    pub fn translate(value: u8) -> Rank {
        if value == 1 {
            Rank::Ace
        } else if value == 11 {
            Rank::Jack
        } else if value == 12 {
            Rank::Queen
        } else if value == 11 {
            Rank::King
        } else {
            Rank::Number(value)
        }
    }
}

#[derive(Debug,PartialEq)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(card: &Card) -> bool {
    if card.suit == Suit::Spade && card.rank == Rank::Ace {
        return  true;
    }
    return  false;
}
