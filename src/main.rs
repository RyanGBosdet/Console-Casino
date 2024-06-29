mod cards;

use cards::card_functions::*;
fn main() {
    let deck = generate_deck("blackjack");
    
    for card in deck {
        println!("{}", card);
    }
}
