use std::env;

const CARGO_SUB_COMMAND: &str = "my";
const ARG_NOTES_AT_FUTURE_ME: &str = "notes@FutureMe";
const ARG_LLVM_COVERAGE_LINK: &str = "llvmCodeCoverageReportLink";

pub enum Action {
    Help,
    NotesAtFutureMe,
    LlvmCoverageLink,
}

/// Returns the requested action based on command line arguments.
pub fn get_action() -> Action {
    let mut args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        return Action::Help;
    }
    if args[0] == CARGO_SUB_COMMAND {
        args.remove(0);
    }
    match args[0].as_str() {
        ARG_NOTES_AT_FUTURE_ME => Action::NotesAtFutureMe,
        ARG_LLVM_COVERAGE_LINK => Action::LlvmCoverageLink,
        _ => Action::Help,
    }
}
