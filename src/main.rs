mod cards;

use cards::card_functions::*;
fn main() {
    let deck = blackjack_deck();

    for card in deck {
        println!("{:?}", card);
    }
}
