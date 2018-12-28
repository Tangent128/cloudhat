CREATE TABLE player (
    id INTEGER PRIMARY KEY NOT NULL,
    urlKey VARCHAR UNIQUE DEFAULT (lower(hex(randomblob(16)))) NOT NULL,
    name VARCHAR DEFAULT ('anon' || lower(hex(randomblob(2)))) NOT NULL
);
