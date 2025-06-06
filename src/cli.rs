use std::env;

const ARG_NOTES_AT_FUTURE_ME: &str = "notes";

pub enum Action {
    Help,
    NotesAtFutureMe,
}

/// Returns the requested action based on command line arguments.
pub fn get_action() -> Action {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Action::Help;
    }
    match args[1].as_str() {
        ARG_NOTES_AT_FUTURE_ME => Action::NotesAtFutureMe,
        _ => Action::Help,
    }
}
