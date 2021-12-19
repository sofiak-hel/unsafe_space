CREATE TABLE IF NOT EXISTS Users (
    id INTEGER PRIMARY KEY,
    username TEXT UNIQUE NOT NULL,
    password TEXT NOT NULL
);

INSERT OR IGNORE INTO Users (username, password) VALUES ("otus", "potus");

CREATE TABLE IF NOT EXISTS Sessions (
    id INTEGER PRIMARY KEY,
    user INTEGER,
    expires INTEGER,
    FOREIGN KEY(user) REFERENCES User(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS Messages (
    id INTEGER PRIMARY KEY,
    user INTEGER,
    content TEXT,
    timestamp INTEGER,
    FOREIGN KEY(user) REFERENCES User(id) ON DELETE CASCADE
);

INSERT OR IGNORE INTO Messages (user, content, timestamp) VALUES (1, "Test message!", 12451)