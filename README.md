# Rustsweeper

A minesweeper clone created in Rust using the Leptos WebAssembly framework.
Run it using the cargo-leptos utility.

Scoreboard data is tracked using sqlite, with the database file path specified in a .env file containing a DATABASE_URL environment variable. Once a file is created and DATABASE_URL is assigned, sqlx migrations will recreate the database.

This project was created using [this](https://github.com/leptos-rs/start-axum) template provided by Leptos.