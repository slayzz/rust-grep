use std::io::{stderr, Write};
use std::process;

const HELP_TEXT: &'static str = "usage grep_rust [arguments] [pattern] [file ...]";

pub fn help_error() {
    stderr().write(format!("{}\n", HELP_TEXT).as_bytes());
    process::exit(2);
}

pub fn invalid_error(value: &str) {
    print_error(&format!("invalid option -- {}\n", value));
}

pub fn print_error(text: &str) {
    stderr().write(text.as_bytes());
    process::exit(1);
}

pub fn print_lines(lines: Vec<String>) {
    lines.into_iter().for_each(|val| print!("{}", val));
}
