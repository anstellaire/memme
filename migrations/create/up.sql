CREATE TABLE decks (
    id INT NOT NULL,
    title TEXT NOT NULL,
    description TEXT NOT NULL,

    PRIMARY KEY (id)
);

CREATE TABLE cards (
    id INT NOT NULL,
    deck_id INT NOT NULL,
    word TEXT NOT NULL,
    translation TEXT NOT NULL,

    PRIMARY KEY (id),
    FOREIGN KEY (deck_id) REFERENCES decks(id)
);
