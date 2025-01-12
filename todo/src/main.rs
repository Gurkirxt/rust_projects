use clap::{Parser, Subcommand};
use rusqlite::{params, Connection, Result};

#[derive(Debug, Parser)]
#[command(name = "TODO App")]
#[command(about = "A simple CLI TODO app", version = "1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Add a new TODO item
    Add {
        /// The description of the TODO item
        description: String,
    },
    /// List all TODO items
    List,
    /// Mark a TODO item as done
    Mark {
        /// The ID of the TODO item to mark as done
        id: i32,
    },
    /// Clear TODO items
    Clear {
        /// Optional ID of the TODO item to delete. If not provided, clears all TODOs.
        id: Option<i32>,
    },
}

#[derive(Debug)]
struct TodoItem {
    id: i32,
    description: String,
    done: bool,
}

fn initialize_database() -> Result<Connection> {
    let conn = Connection::open("todo.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS todo (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            description TEXT NOT NULL,
            done BOOLEAN NOT NULL DEFAULT 0
        )",
        [],
    )?;
    Ok(conn)
}

fn add_todo_item(conn: &Connection, description: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO todo (description, done) VALUES (?1, 0)",
        params![description],
    )?;
    println!("TODO added: {}", description);
    Ok(())
}

fn list_todo_items(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("SELECT id, description, done FROM todo")?;
    let todo_iter = stmt.query_map([], |row| {
        Ok(TodoItem {
            id: row.get(0)?,
            description: row.get(1)?,
            done: row.get(2)?,
        })
    })?;

    println!("TODO List:");
    for todo in todo_iter {
        let todo = todo?;
        println!(
            "[{}] {} - {}",
            todo.id,
            if todo.done { "x" } else { " " },
            todo.description
        );
    }
    Ok(())
}

fn mark_todo_done(conn: &Connection, id: i32) -> Result<()> {
    let updated = conn.execute("UPDATE todo SET done = 1 WHERE id = ?1", params![id])?;
    if updated > 0 {
        println!("TODO with ID {} marked as done.", id);
    } else {
        println!("No TODO found with ID {}.", id);
    }
    Ok(())
}

fn clear_todo_items(conn: &Connection, id: Option<i32>) -> Result<()> {
    match id {
        Some(id) => {
            // Delete a specific TODO item by ID
            let rows_deleted = conn.execute("DELETE FROM todo WHERE id = ?1", params![id])?;
            if rows_deleted > 0 {
                println!("TODO with ID {} has been deleted.", id);
            } else {
                println!("No TODO found with ID {}.", id);
            }
        }
        None => {
            // Delete all TODO items
            conn.execute("DELETE FROM todo", [])?;
            println!("All TODO items have been cleared.");
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let conn = initialize_database()?;

    match &cli.command {
        Commands::Add { description } => {
            add_todo_item(&conn, description)?;
        }
        Commands::List => {
            list_todo_items(&conn)?;
        }
        Commands::Mark { id } => {
            mark_todo_done(&conn, *id)?;
        }
        Commands::Clear { id } => {
            clear_todo_items(&conn, *id)?;
        }
    }

    Ok(())
}
