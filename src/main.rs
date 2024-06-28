mod cards;

use cards::card_functions::*;
fn main() {
    let mut deck = generate_deck("blackjack");
    let deck_ref = &mut deck;
    shuffle_deck(deck_ref);
    
    for card in deck {
        println!("{}", card);
    }
}
