use crate::storage;
use crate::frontend;

pub struct CliView {}

impl frontend::traits::View for CliView {
    fn notify(&self, message: &str) {
        println!("{}", message);
    }

    fn present_decks(&self, decks: &std::vec::Vec<storage::models::Deck>) {
        println!("Decks ({} results):", decks.len());

        println!("{:<8} {:<20} {:<32}",
            "ID", "TITLE", "DESCRIPTION");
        for deck in decks {
            println!("{:<8} {:<20} {:<32}",
                deck.id, deck.title, deck.description);
        }
    }

    fn present_cards(&self, cards: &std::vec::Vec<storage::models::Card>) {
        println!("Cards ({} results):", cards.len());

        println!("{:<8} {:<8} {:<22} {:<22}",
            "ID", "DECK_ID", "WORD", "TRANSLATION");
        for card in cards {
            println!("{:<8} {:<8} {:<22} {:<22}",
                card.id, card.deck_id, card.word, card.translation);
        }
    }
}
