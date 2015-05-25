use rand;
use rand::{Rng};

use card::{Suit, Value, Card};

pub struct Deck {
    count_dealt: usize,
    cards: [u16; 52],
}

pub enum DeckError {
    NotEnoughCards
}

fn create_card_for_value(value: &u16) -> Card {
    let suit = match value/13 {
        0 => Suit::Spades,
        1 => Suit::Hearts,
        2 => Suit::Diamonds,
        3 => Suit::Clubs,
        _ => panic!("Unexpected suit conversion number")
    };

    let value = match value%13 {
        0 => Value::Two,
        1 => Value::Three,
        2 => Value::Four,
        3 => Value::Five,
        4 => Value::Six,
        5 => Value::Seven,
        6 => Value::Eight,
        7 => Value::Nine,
        8 => Value::Ten,
        9 => Value::Jack,
        10 => Value::Queen,
        11 => Value::King,
        12 => Value::Ace,
        _ => panic!("Unexpected value conversion number")
    };

    Card(value, suit)
}

impl Deck {
    //TODO: a deck containing multiple sets of cards? When 52*3 is needed.

    pub fn new_unshuffled() -> Deck {
        let mut d = Deck {
            count_dealt: 0,
            cards: [0; 52],
        };

        let mut value = 0;
        for x in d.cards.iter_mut() {
            *x = value;
            value += 1;
        }
        d
    }

    pub fn new_shuffled() -> Deck {
        let mut d = Deck::new_unshuffled();
        d.shuffle();
        d
    }

    pub fn reset_unshuffled(&mut self) {
        self.count_dealt = 0;
    }

    pub fn reset_shuffled(&mut self) {
        self.count_dealt = 0;
        self.shuffle();
    }

    fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        rng.shuffle(&mut self.cards);
    }

    pub fn draw(&mut self) -> Result<Card, DeckError> {
        if (self.count_dealt + 1) > 52 {
            Err(DeckError::NotEnoughCards)
        } else {
            let value = &self.cards[self.count_dealt];
            self.count_dealt+=1;

            let card = create_card_for_value(value);
            Ok(card)
        }
    }

    pub fn draw_n(&mut self, n: &usize) -> Result<Vec<Card>, DeckError> {
        if (self.count_dealt + n) > 52 {
            Err(DeckError::NotEnoughCards)
        } else {
            let mut cards = Vec::new();

            for _ in 0..*n {
                cards.push(self.draw().ok().unwrap());
            }

            Ok(cards)
        }
    }
}

