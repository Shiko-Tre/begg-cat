use rusqlite::{Connection, Result};
use std::fs;

fn main() -> Result<()> {
    // 1. Create SQLite DB (can also be a file like "database.db")
    let conn = Connection::open_in_memory()?;
    
    conn.execute(
        "CREATE TABLE games (id INTEGER PRIMARY KEY, title TEXT)",
        [],
    )?;
    conn.execute(
        "INSERT INTO games (title) VALUES (?1)",
        ["Proof of Concept: Rust + SQLite on GitHub Pages works!"],
    )?;

    // 2. Query SQLite
    let mut stmt = conn.prepare("SELECT title FROM games")?;
    let game_titles = stmt.query_map([], |row| row.get::<_, String>(0))?
        .collect::<Result<Vec<_>, _>>()?;

    // 3. Generate HTML output for GitHub Pages
    fs::create_dir_all("dist").unwrap();
    let items = game_titles.iter().map(|g| format!("<li>{}</li>", g)).collect::<String>();
    let html = format!(
        "<!DOCTYPE html><html><head><title>Game DB</title></head><body><h1>Game Database</h1><ul>{}</ul></body></html>",
        items
    );

    fs::write("dist/index.html", html).unwrap();
    println!("Built dist/index.html successfully!");
    Ok(())
}
