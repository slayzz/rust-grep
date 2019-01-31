
use std::env;
use std::collections::HashSet;

use regex::Regex;

use crate::utils;

//#[derive(Debug)]
//pub enum Args {
//    Invert,
//    LineNumber,
//    Invalid,
//}
//
//impl Args {
//    fn new(value: &str) -> Args {
//        match value {
//            "v" => Args::Invert,
//            "n" => Args::LineNumber,
//            _ => {
//                utils::invalid_error(&value);
//                Args::Invalid
//            }
//        }
//    }
//}

#[derive(Debug)]
pub enum ProgrammState {
    ReadFromInput,
    ReadFromFile
}

#[derive(Debug)]
pub struct Config {
    pub arguments: HashSet<char>,
    pub file_name: String,
    pub pattern: String,
    pub state: ProgrammState,
}

impl Config {
    pub fn new() -> Config {
        let mut args_strings: Vec<String> = env::args().skip(1).collect();

        let arguments = args_strings.iter()
            .filter(|arg| arg.contains("-"))
            .flat_map(|arg| arg.chars().skip(1))
            .collect();

        remove_by_pattern(&mut args_strings, Regex::new("-").unwrap());

        if args_strings.len() == 0 {
            utils::help_error();
        }

        let pattern = args_strings[0].clone();
        args_strings.remove(0);

        if args_strings.len() == 0 {
            return Config {
                arguments,
                file_name: String::from(""),
                pattern,
                state: ProgrammState::ReadFromInput
            };
        }

        let file_name = args_strings[0].clone();

        Config {
            arguments,
            file_name: file_name,
            pattern,
            state: ProgrammState::ReadFromFile
        }
    }

}

fn remove_by_pattern(removable_list: &mut Vec<String>, pattern: Regex) {
    loop {
        let position = removable_list.iter().position(|arg| pattern.is_match(arg));
        if let Some(position) = position {
            removable_list.remove(position);
        } else {
            break;
        }
    }
}