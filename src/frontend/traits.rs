use crate::storage;

pub trait View {
    fn notify(&self, message: &str);
    fn present_decks(&self, decks: &std::vec::Vec<storage::models::Deck>);
    fn present_cards(&self, cards: &std::vec::Vec<storage::models::Card>);
}
