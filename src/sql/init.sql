CREATE TABLE IF NOT EXISTS Users (
    id INTEGER PRIMARY KEY,
    username TEXT NOT NULL,
    password TEXT NOT NULL
);

INSERT INTO Users (username, password) VALUES ("otus", "potus");

CREATE TABLE IF NOT EXISTS Sessions (
    id INTEGER PRIMARY KEY,
    user INTEGER,
    expires INTEGER,
    FOREIGN KEY(user) REFERENCES User(id) ON DELETE CASCADE
);