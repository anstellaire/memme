use super::models;

pub trait StorageManagable {
    fn create_deck(&self, deck: &models::NewDeck) -> Result<usize,()>;
    fn create_cards(&self, cards: &Vec<models::NewCard>) -> Result<usize,()>;

    fn read_decks(&self) -> Result<Vec<models::Deck>,()>;
    fn read_cards(&self) -> Result<Vec<models::Card>,()>;
    fn read_cards_in_deck(&self, deck_id: i32) -> Result<Vec<models::Card>,()>;
}
