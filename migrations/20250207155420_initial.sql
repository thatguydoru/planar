CREATE TABLE IF NOT EXISTS user (
    id INTEGER PRIMARY KEY NOT NULL,
    username VARCHAR(255)
);

CREATE TABLE IF NOT EXISTS board (
    id INTEGER PRIMARY KEY NOT NULL,
    owner INTEGER NOT NULL,

    FOREIGN KEY(owner) REFERENCES user(id),

    title VARCHAR(255),
    description TEXT
);

CREATE TABLE IF NOT EXISTS column (
    id INTEGER PRIMARY KEY NOT NULL,
    owner INTEGER NOT NULL,
    board INTEGER NOT NULL,

    FOREIGN KEY(owner) REFERENCES user(id),
    FOREIGN KEY(board) REFERENCES board(id),

    title VARCHAR(255),
    description TEXT,
);
