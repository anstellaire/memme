use crate::storage;
use crate::backend;

use std::convert::TryInto;

const MENU: &'static str =
"
Options:
1 Show Decks
2 Show Cards
3 Show Cards in Deck
4 Add Deck
5 Add Cards
0 Exit
";

mod details {
    use std::io::Write;

    pub fn print(msg: &str) {
        print!("{}", msg);
        std::io::stdout().flush().unwrap();
    }

    pub fn read_line() -> String {
        let mut line : String = String::new();

        std::io::stdin().read_line(&mut line)
            .expect("ERROR: Failed to read line");

        return line;
    }

    pub fn read_uint() -> Result<u32, std::num::ParseIntError> {
        let line = read_line();
        return line.trim().parse::<u32>();
    }

    pub fn should_continue() -> bool {
        loop {
            print("Continue? (Y/n): ");

            match read_line().trim() {
                "Y" | "y" | "" => return true,
                "n" => return false,
                _ => println!("Value should be Y or n"),
            }
        }
    }
}

pub struct Input<'a, C>
where
    C: backend::traits::Controller
{
    pub ctrl: &'a C,
}

impl<'a, C> Input<'a, C>
where
    C: backend::traits::Controller
{
    pub fn show(&self) {
        loop {
            details::print(MENU);
            details::print("> Enter option number: ");

            match details::read_uint() {
                Ok(0u32) => std::process::exit(0),
                Ok(1u32) => self.ctrl.read_decks(),
                Ok(2u32) => self.ctrl.read_cards(),
                Ok(3u32) => self.ctrl.read_cards_in_deck(self.request_deck_id()),
                Ok(4u32) => self.ctrl.create_deck(self.request_deck()),
                Ok(5u32) => self.ctrl.create_cards(self.request_cards()),
                _ => println!("Invalid input, try again..."),
            };
        }
    }

    pub fn request_deck_id(&self) -> i32 {
        details::print("> Enter Deck ID: ");
        let deck_id : i32 = details::read_uint()
            .expect("ERROR: Failed to read number (must be integral)")
            .try_into()
            .expect("ERROR: Failed to read number (must be positive)");

        return deck_id;
    }

    pub fn request_deck(&self) -> storage::models::Deck {
        details::print("> Enter Deck id: ");
        let id : i32 = details::read_uint()
            .expect("ERROR: Failed to read number (must be integral)")
            .try_into()
            .expect("ERROR: Failed to read number (must be positive)");

        details::print("> Enter Deck title: ");
        let title = details::read_line().trim().to_string();

        details::print("> Enter Deck decription: ");
        let description = details::read_line().trim().to_string();

        return storage::models::Deck{id, title, description};
    }

    pub fn request_card(&self) -> storage::models::Card {
        details::print("> Enter Card id: ");
        let id : i32 = details::read_uint()
            .expect("ERROR: Failed to read number (must be integral)")
            .try_into()
            .expect("ERROR: Failed to read number (must be positive)");

        details::print("> Enter Deck id: ");
        let deck_id : i32 = details::read_uint()
            .expect("ERROR: Failed to read number (must be integral)")
            .try_into()
            .expect("ERROR: Failed to read number (must be positive)");

        details::print("> Enter word: ");
        let word = details::read_line().trim().to_string();

        details::print("> Enter translation: ");
        let translation = details::read_line().trim().to_string();

        return storage::models::Card{id, deck_id, word, translation};
    }

    pub fn request_cards(&self) -> Vec<storage::models::Card> {
        let mut cards : Vec<storage::models::Card> = Vec::new();

        loop {
            cards.push(self.request_card());

            if !details::should_continue() { break; }
        }

        return cards;
    }
}
