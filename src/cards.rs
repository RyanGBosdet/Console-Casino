pub mod card_functions {

    #[derive(Debug, Clone)]
    pub enum Suit {
        Hearts(String),
        Diamonds(String),
        Clubs(String),
        Spades(String),
        None
    }

    #[derive(Debug, Clone)]
    pub enum Name {
        Two,
        Three,
        Four,
        Five,
        Six,
        Seven,
        Eight,
        Nine,
        Ten,
        Jack,
        Queen, 
        King,
        Ace,
        Empty,
    }

    #[derive(Debug, Clone)]
    pub struct Card {
        name: Name,
        suit: Suit,
        value: i32
    }

    fn card_factory(name: Name, suit: Suit, value: i32) -> Card {
        Card {name, suit, value}
    }

    pub fn blackjack_deck() -> [Card; 52] {
        use Name::*;
        use Suit::*;

        const EMPTY_CARD: Card = Card {
            name: Empty,
            suit: None,
            value: 0,
        };

        let mut card_array: [Card; 52] = [EMPTY_CARD; 52];

        let suits = [
            Hearts(String::from("♥")), 
            Diamonds(String::from("♦")), 
            Clubs(String::from("♣")), 
            Spades(String::from("♠"))
        ];

        let names = [
            Two,
            Three,
            Four,
            Five,
            Six,
            Seven,
            Eight,
            Nine,
            Ten,
            Jack,
            Queen,
            King,
            Ace,
        ];

        let values = [
            2,
            3,
            4,
            5,
            6,
            7,
            8,
            9,
            10,
            10,
            10,
            10,
            11
        ];

        let mut index = 0;

        for i in suits.iter() {
            for j in 0..13 {
                card_array[index] = card_factory(names[j].clone(), i.clone(), values[j]);
                index += 1;
            }
        }

        card_array

    }

}