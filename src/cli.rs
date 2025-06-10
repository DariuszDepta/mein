//! # Command-line actions

const ARG_NOTES_AT_FUTURE_ME: &str = "notes@FutureMe";
const ARG_LLVM_COVERAGE_LINK: &str = "llvmCodeCoverageReportLink";

pub enum Action {
  Help,
  NotesAtFutureMe,
  LlvmCoverageLink,
}

/// Returns the requested action based on command line arguments.
pub fn get_action(args: Vec<String>) -> Action {
  if args.is_empty() {
    return Action::Help;
  }
  match args[0].as_str() {
    ARG_NOTES_AT_FUTURE_ME => Action::NotesAtFutureMe,
    ARG_LLVM_COVERAGE_LINK => Action::LlvmCoverageLink,
    _ => Action::Help,
  }
}
