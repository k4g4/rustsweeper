CREATE TABLE scores(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT NOT NULL,
    time_in_seconds INTEGER NOT NULL,
    difficulty TEXT NOT NULL,
    size TEXT NOT NULL
);