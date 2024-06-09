use serde::Deserialize;
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
enum UserError {
    #[error("No users found")]
    NoUsers,
    #[error("Too many users found")]
    TooManyUsers,
}

fn maybe_read_a_file() -> Result<String, std::io::Error> {
    let my_file = Path::new("my_file.txt");
    std::fs::read_to_string(my_file)
}

fn file_to_uppercase() -> Result<String, std::io::Error> {
    let contents = maybe_read_a_file()?;
    Ok(contents.to_uppercase())
}

#[derive(Deserialize)]
struct User {
    user: String,
}

type GenericResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn load_users() -> Result<Vec<User>, UserError> {
    let my_path = Path::new("users.json");
    let raw_text = std::fs::read_to_string(my_path).map_err(|_| UserError::NoUsers)?;
    let users: Vec<User> = serde_json::from_str(&raw_text).map_err(|_| UserError::NoUsers)?;

    // anyhow::bail!("Oh no! we can't go on!");
    Ok(users)
}

fn main() {
    // let s: Result<String, std::io::Error> = file_to_uppercase();
    if let Ok(contents) = file_to_uppercase() {
        println!("Contents {contents}");
    }

    let my_file = Path::new("my_file.txt");
    let content: Result<String, std::io::Error> = std::fs::read_to_string(my_file);
    match content {
        Ok(contents) => println!("{contents}"),
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => println!("File not found -- my_file.txt"),
            _ => println!("ERROR: {e:#?}"),
        },
    }
}
