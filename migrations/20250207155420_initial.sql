CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY NOT NULL,
    username VARCHAR(255) UNIQUE NOT NULL
);

CREATE TABLE IF NOT EXISTS boards (
    id INTEGER PRIMARY KEY NOT NULL,
    owner INTEGER NOT NULL,
    title VARCHAR(255) NOT NULL,
    description TEXT,

    FOREIGN KEY(owner) REFERENCES users(id)
);

CREATE TABLE IF NOT EXISTS columns (
    id INTEGER PRIMARY KEY NOT NULL,
    owner INTEGER NOT NULL,
    board INTEGER NOT NULL,
    title VARCHAR(255) NOT NULL,
    description TEXT,

    FOREIGN KEY(owner) REFERENCES users(id),
    FOREIGN KEY(board) REFERENCES boards(id)
);

CREATE TABLE IF NOT EXISTS cards (
    id INTEGER PRIMARY KEY NOT NULL,
    owner INTEGER NOT NULL,
    board INTEGER NOT NULL,
    column INTEGER NOT NULL,
    title VARCHAR(255) NOT NULL,
    description TEXT,

    FOREIGN KEY (owner) REFERENCES users(id),
    FOREIGN KEY (board) REFERENCES boards(id),
    FOREIGN KEY (column) REFERENCES columns(id)
);
