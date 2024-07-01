mod cards;

use cards::card_functions::*;
use terminal_menu::{menu, button, run, label, mut_menu};
use crossterm::style::Color;

fn main() {
    let my_menu = menu(vec![
        label("Welcome to the console casino! What would you like to play?"),
        button("Blackjack").colorize(Color::Red),
        button("Poker").colorize(Color::Blue),
        button("Baccarat").colorize(Color::Red),
        button("Roulette").colorize(Color::Blue),
        button("Slots").colorize(Color::Red),
    ]);
    run(&my_menu);
    println!("selected item name: {}", mut_menu(&my_menu).selected_item_name());

}
