//! # Application implementation

use crate::actions;
use crate::cli::*;

pub fn run(args: Vec<String>) {
  match get_action(args) {
    Action::Help => {
      println!("printing help")
    }
    Action::NotesAtFutureMe => actions::notes_at_future_me::print(),
    Action::LlvmCoverageLink => actions::llvm_coverage_link::print(),
  }
}
