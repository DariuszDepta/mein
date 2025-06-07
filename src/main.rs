//! # Mein Schweizer Taschenmesser

mod actions;
mod cli;

use actions::*;
use cli::*;

fn main() {
    match get_action() {
        Action::Help => {
            println!("printing help")
        }
        Action::NotesAtFutureMe => action_notes_at_future_me::print(),
    }
}
