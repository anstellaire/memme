use crate::storage;
use crate::backend;
use crate::frontend;

pub struct DefaultController<'a, D, V>
where
    D: storage::traits::DataProvider,
    V: frontend::traits::View
{
    pub conn: &'a D,
    pub view: &'a V
}

impl<'a, D, V> backend::traits::Controller for DefaultController<'a, D, V>
where
    D: storage::traits::DataProvider,
    V: frontend::traits::View
{
    fn read_decks(&self) {
        self.view.notify("Reading decks...");

        let decks = self.conn.read_decks()
            .expect("ERROR: Failed to read decks");

        self.view.present_decks(&decks);
    }

    fn read_cards(&self) {
        self.view.notify("Reading cards...");

        let cards = self.conn.read_cards()
            .expect("ERROR: Failed to read cards");

        self.view.present_cards(&cards);
    }

    fn read_cards_in_deck(&self, deck_id: i32) {
        self.view.notify("Reading cards in the deck...");

        let cards = self.conn.read_cards_in_deck(deck_id)
            .expect("ERROR: Failed to read cards in the deck");

        self.view.present_cards(&cards);
    }

    fn create_deck(&self, deck: storage::models::Deck) {
        self.view.notify("Creating the deck...");

        let new_deck = storage::models::NewDeck{
            id: deck.id,
            title: &deck.title,
            description: &deck.description
        };

        self.conn.create_deck(&new_deck)
            .expect("ERROR: Failed to insert the deck");
    }

    fn create_cards(&self, cards: Vec<storage::models::Card>) {
        self.view.notify("Creating the cards...");

        let mut new_cards = Vec::<storage::models::NewCard>::new();
        for card in &cards {
            new_cards.push(storage::models::NewCard{
                id: card.id,
                deck_id: card.deck_id,
                word: &card.word,
                translation: &card.translation
            });
        }

        self.conn.create_cards(&new_cards)
            .expect("ERROR: Failed to insert the card");
    }
}
