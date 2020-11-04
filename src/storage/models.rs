use super::schema::*;

#[derive(Queryable)]
pub struct Deck {
    pub id: i32,
    pub title: String,
    pub description: String,
}

#[derive(Queryable)]
pub struct Card {
    pub id: i32,
    pub deck_id: i32,
    pub word: String,
    pub translation: String,
}

#[derive(Insertable)]
#[table_name="decks"]
pub struct NewDeck<'a> {
    pub id: i32,
    pub title: &'a str,
    pub description: &'a str,
}

#[derive(Insertable)]
#[table_name="cards"]
pub struct NewCard<'a> {
    pub id: i32,
    pub deck_id: i32,
    pub word: &'a str,
    pub translation: &'a str,
}
