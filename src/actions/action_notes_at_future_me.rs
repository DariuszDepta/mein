const NOTES_AT_FUTURE_ME: &str = include_str!("../templates/NOTES_AT_FUTURE_ME.md");

pub fn print() {
    println!("{}", NOTES_AT_FUTURE_ME);
}
