use crate::storage;

pub trait Controller {
    fn read_decks(&self);
    fn read_cards(&self);
    fn read_cards_in_deck(&self, deck_id: i32);
    fn create_deck(&self, deck: storage::models::Deck);
    fn create_cards(&self, cards: Vec<storage::models::Card>);
}
