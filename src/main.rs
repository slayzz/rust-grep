extern crate regex;

mod executor;
mod utils;
mod parser;

use crate::parser::{Config};
use crate::executor::{Executor};

fn main() {
    let config = Config::new();
    let executor = Executor::new(config);
    executor.run();
}
