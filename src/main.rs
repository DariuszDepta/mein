//! # Mein Schweizer Taschenmesser

mod cli;

use cli::*;

const NOTES_AT_FUTURE_ME: &str = include_str!("./templates/NOTES_AT_FUTURE_ME.md");

fn main() {
    match get_action() {
        Action::Help => {
            println!("printing help")
        }
        Action::NotesAtFutureMe => {
            println!("{}", NOTES_AT_FUTURE_ME);
        }
    }
}
