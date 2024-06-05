use std::collections::HashMap;
use std::error::Error;
use rusqlite::{Connection, Result};

#[derive(Debug)]
struct Cat {
    name: String,
    color: String,
}

// https://rust-lang-nursery.github.io/rust-cookbook/database/sqlite.html
fn main() -> Result<()> {
    let conn = Connection::open("data/cats.db")?;
    conn.execute("create table if not exists cat_colors (\
        id integer primary key,
        name text not null unique
    )", ())?;

    conn.execute("create table if not exists cats (\
    id integer primary key,
    name text not null,
    color_id integer not null references cat_colors(id))
    ", ())?;

    let mut cat_colors = HashMap::new();
    cat_colors.insert("Blue".to_string(), vec!["Tiger", "Sammy"]);
    cat_colors.insert("Black".to_string(), vec!["Oreo", "Biscuit"]);

    for (color, catnames) in &cat_colors {
        conn.execute(
            "INSERT INTO cat_colors (name) values (?1)",
            &[&color.to_string()],
        )?;
        let last_id: String = conn.last_insert_rowid().to_string();

        for cat in catnames {
            conn.execute(
                "INSERT INTO cats (name, color_id) values (?1, ?2)",
                &[&cat.to_string(), &last_id],
            )?;
        }
    }
    let mut stmt = conn.prepare(
        "SELECT c.name, cc.name from cats c
         INNER JOIN cat_colors cc
         ON cc.id = c.color_id;",
    )?;

    let cats = stmt.query_map((), |row| {
        Ok(Cat {
            name: row.get(0)?,
            color: row.get(1)?,
        })
    })?;

    for cat in cats {
        println!("Found cat {:?}", cat);
    }


    Ok(())
}
