//! # Binary run as Cargo utility

use std::env;

mod actions;
mod app;
mod cli;

fn main() {
  app::run(env::args().skip(2).collect());
}
