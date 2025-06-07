//! # Mein Schweizer Taschenmesser

mod actions;
mod cli;

use cli::*;

fn main() {
    println!("{:?}", std::env::args().collect::<Vec<String>>());
    match get_action() {
        Action::Help => {
            println!("printing help")
        }
        Action::NotesAtFutureMe => actions::notes_at_future_me::print(),
        Action::LlvmCoverageLink => actions::llvm_coverage_link::print(),
    }
}
