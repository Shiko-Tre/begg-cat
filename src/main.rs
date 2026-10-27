use rusqlite::{Connection, Result};
use std::fs;

fn main() -> Result<()> {
    // 1. Create SQLite DB
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

    // 3. Generate HTML output with a black dark-theme
    fs::create_dir_all("dist").unwrap();
    let items = game_titles.iter().map(|g| format!("<li>{}</li>", g)).collect::<String>();
    
    let html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Game DB</title>
    <style>
        body {{
            background-color: #0d1117;
            color: #c9d1d9;
            font-family: system-ui, -apple-system, sans-serif;
            padding: 2rem;
            margin: 0;
        }}
        h1 {{
            color: #58a6ff;
            border-bottom: 1px solid #30363d;
            padding-bottom: 0.5rem;
        }}
        ul {{
            line-height: 1.6;
        }}
    </style>
</head>
<body>
    <h1>Game Database</h1>
    <ul>{}</ul>
</body>
</html>"#,
        items
    );

    fs::write("dist/index.html", html).unwrap();
    println!("Built dist/index.html with dark theme successfully!");
    Ok(())
}
