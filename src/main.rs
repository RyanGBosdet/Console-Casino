mod cards;

use cards::card_functions::*;
fn main() {
    let mut deck = blackjack_deck();
    let deck_add = &mut deck;
    shuffle_deck(deck_add);
    
    for card in deck {
        println!("{:?}", card);
    }
}
