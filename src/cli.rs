use std::env;

const ARG_NOTES_AT_FUTURE_ME: &str = "notes@FutureMe";
const ARG_LLVM_COVERAGE_LINK: &str = "llvmCodeCoverageReportLink";

pub enum Action {
    Help,
    NotesAtFutureMe,
    LlvmCoverageLink,
}

/// Returns the requested action based on command line arguments.
pub fn get_action() -> Action {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Action::Help;
    }
    match args[1].as_str() {
        ARG_NOTES_AT_FUTURE_ME => Action::NotesAtFutureMe,
        ARG_LLVM_COVERAGE_LINK => Action::LlvmCoverageLink,
        _ => Action::Help,
    }
}
