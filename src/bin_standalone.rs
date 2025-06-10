//! # Binary run as standalone application

use std::env;

mod actions;
mod app;
mod cli;

fn main() {
  app::run(env::args().skip(1).collect());
}
