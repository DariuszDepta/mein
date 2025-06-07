use antex::{ColorMode, StyledText, Text};
use std::env;

pub fn print() {
    match env::current_dir() {
        Ok(path) => {
            Text::new(ColorMode::On)
                .green()
                .bold()
                .nl()
                .s("Open coverage report")
                .colon()
                .space()
                .clear()
                .s("file://")
                .s(path.display())
                .s("/target/llvm-cov/html/index.html")
                .nl()
                .cprintln();
        }
        Err(reason) => println!("Error getting current directory: {reason}"),
    }
}
