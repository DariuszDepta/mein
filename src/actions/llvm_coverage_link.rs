use antex::{ColorMode, StyledText, Text};
use std::env;

pub fn print() {
  match env::current_dir() {
    Ok(mut path) => {
      path = path.join("target/llvm-cov/html/index.html");
      if path.exists() {
        Text::new(ColorMode::On)
          .yellow()
          .bold()
          .nl()
          .s("Open code coverage report")
          .colon()
          .space()
          .clear()
          .s("file://")
          .s(path.display())
          .nl()
          .cprintln();
      } else {
        Text::new(ColorMode::On).yellow().bold().nl().s("Coverage report not found!").nl().cprintln();
      }
    }
    Err(reason) => eprintln!("Error getting current directory: {reason}"),
  }
}
