pub mod card_functions {
    use rand::seq::SliceRandom;
    use rand::thread_rng;
    use std::fmt;

    #[derive(Debug, Clone)]
    enum Suit {
        Hearts(String),
        Diamonds(String),
        Clubs(String),
        Spades(String),
        None
    }

    impl Suit {
        pub fn unwrap(&self) -> String {
            use Suit::*;

            match self {
                Hearts(x) => x.to_string(),
                Diamonds(x) => x.to_string(),
                Clubs(x) => x.to_string(),
                Spades(x) => x.to_string(),
                None => String::from("No suit found")
            }
        }
    }

    #[derive(Debug, Clone)]
    enum Name {
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

    impl fmt::Display for Card {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            use Name::*;

            let suit_ref = &self.suit;
            
            match self.name {
                Two => write!(f, "2{}", suit_ref.unwrap()),
                Three => write!(f, "3{}", suit_ref.unwrap()),
                Four => write!(f, "4{}", suit_ref.unwrap()),
                Five => write!(f, "5{}", suit_ref.unwrap()),
                Six => write!(f, "6{}", suit_ref.unwrap()),
                Seven => write!(f, "7{}", suit_ref.unwrap()),
                Eight => write!(f, "8{}", suit_ref.unwrap()),
                Nine => write!(f, "9{}", suit_ref.unwrap()),
                Ten => write!(f, "10{}", suit_ref.unwrap()),
                Jack => write!(f, "J{}", suit_ref.unwrap()),
                Queen => write!(f, "Q{}", suit_ref.unwrap()),
                King => write!(f, "K{}", suit_ref.unwrap()),
                Ace => write!(f, "A{}", suit_ref.unwrap()),
                Empty => write!(f, "Empty Card"),
            }
        }
    }

    fn card_factory(name: Name, suit: Suit, value: i32) -> Card {
        Card {name, suit, value}
    }

    pub fn shuffle_deck(slice: &mut [Card]) {
        let mut rng = thread_rng();
        slice.shuffle(&mut rng);
    }

    pub fn generate_deck(game: &str) -> [Card; 52] {
        use Name::*;
        use Suit::*;

        const EMPTY_CARD: Card = Card {
            name: Empty,
            suit: None,
            value: 0,
        };

        let mut card_array: [Card; 52] = [EMPTY_CARD; 52];
        let card_ref = &mut card_array;

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
        
        match game {
            "blackjack" => {
                    build_deck(card_ref, suits, names, [2,3,4,5,6,7,8,9,10,10,10,10,11]);
                    shuffle_deck(card_ref);
                    card_array
                },
            "baccarat" => {
                build_deck(card_ref, suits, names, [2,3,4,5,6,7,8,9,0,0,0,0,1]);
                shuffle_deck(card_ref);
                card_array
                },
            "poker" => {
                build_deck(card_ref, suits, names, [2,3,4,5,6,7,8,9,10,11,12,13,14]);
                shuffle_deck(card_ref);
                card_array
            },
            _ => card_array
        }
    }


    fn build_deck(card_array: &mut [Card; 52], suits: [Suit; 4], names: [Name; 13], values: [i32; 13]) { 

        let mut index = 0;

        for i in suits.iter() {
            for j in 0..13 {
                card_array[index] = card_factory(names[j].clone(), i.clone(), values[j]);
                index += 1;
            }
        }

    }
    

}