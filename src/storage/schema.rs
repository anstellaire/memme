table! {
    cards (id) {
        id -> Integer,
        deck_id -> Integer,
        word -> Text,
        translation -> Text,
    }
}

table! {
    decks (id) {
        id -> Integer,
        title -> Text,
        description -> Text,
    }
}

joinable!(cards -> decks (deck_id));

allow_tables_to_appear_in_same_query!(
    cards,
    decks,
);
