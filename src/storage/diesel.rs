// crates
extern crate diesel;

// traits
use diesel::Connection;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use diesel::ExpressionMethods;
use diesel::TextExpressionMethods;

use super::traits::StorageManagable;

// modules
use super::schema; // for StorageManagable trait impl
use super::models; // for StorageManagable trait impl

// -----------------------------------------------------------------------------
//   DieselConnectionManager
// -----------------------------------------------------------------------------

pub struct DieselConnectionManager {
    connection: diesel::SqliteConnection
}

// -----------------------------------------------------------------------------
//   DieselConnectionManager: General Implementation
// -----------------------------------------------------------------------------

impl DieselConnectionManager {
    pub fn establish(dbname: &str) -> DieselConnectionManager {
        let connection =
            diesel::SqliteConnection::establish(dbname).unwrap_or_else(|_|{

            eprintln!("ERROR: failed to connect to '{}'", dbname);
            std::process::exit(1);
        });

        return DieselConnectionManager{connection};
    }
}

// -----------------------------------------------------------------------------
//   DieselConnectionManager: StorageManagable Trait Implementation
// -----------------------------------------------------------------------------

impl StorageManagable for DieselConnectionManager {
    fn create_deck(&self, deck: &models::NewDeck) -> Result<usize,()> {
        let res = diesel::insert_into(schema::decks::dsl::decks)
            .values(deck)
            .execute(&self.connection);

        match res {
            Ok(val) => Ok(val),
            Err(_) => Err(())
        }
    }

    fn create_cards(&self, cards: &Vec<models::NewCard>) -> Result<usize,()> {
        let res = diesel::insert_into(schema::cards::dsl::cards)
            .values(cards)
            .execute(&self.connection);

        match res {
            Ok(val) => Ok(val),
            Err(_) => Err(())
        }
    }

    fn read_decks(&self) -> Result<Vec<models::Deck>,()> {
        let res = schema::decks::dsl::decks
            .load::<models::Deck>(&self.connection);

        match res {
            Ok(val) => Ok(val),
            Err(_) => Err(())
        }
    }

    fn read_cards(&self) -> Result<Vec<models::Card>,()> {
        let res = schema::cards::dsl::cards
            .load::<models::Card>(&self.connection);

        match res {
            Ok(val) => Ok(val),
            Err(_) => Err(())
        }
    }

    fn read_cards_in_deck(&self, deck_id: i32) -> Result<Vec<models::Card>,()> {
        let res = schema::cards::dsl::cards
            .filter(schema::cards::dsl::deck_id.eq(deck_id))
            .load::<models::Card>(&self.connection);

        match res {
            Ok(val) => Ok(val),
            Err(_) => Err(())
        }
    }
}
