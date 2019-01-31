
use std::fs::{File};
use std::io::{Read, BufReader, BufRead};
// use std::error::Error;
// use std::thread;
// use std::time::Duration;

use crate::parser::{Config, ProgrammState};
use crate::utils;

pub struct Executor {
   config: Config,
   file_lines: Vec<String>,
}

impl Executor {
   pub fn new(config: Config) -> Self {
       Self { config, file_lines: vec![] }
   }

   pub fn run(self) {
       let Config { file_name, state, .. } = &self.config;

       match state {
           ProgrammState::ReadFromFile => {
               let buf_reader = self.open_to_read(File::open(file_name).unwrap());
               self.search(buf_reader);
           },
           ProgrammState::ReadFromInput => {
               let buf_reader = self.open_to_read(std::io::stdin());
               self.search(buf_reader);
           } 
       }
   }

   fn search<T: Read>(mut self, mut buf_reader: BufReader<T>) {
        let mut buf = String::new();

        while let Ok(count) = buf_reader.read_line(&mut buf) {
            if count == 0 {
                break;
            }
            self.file_lines.push(buf.clone());
            buf.clear();
        }

        let searched_lines: Vec<String> = self.file_lines
            .iter()
            .filter_map(|val| if val.contains(&self.config.pattern) { Some(val.to_owned()) } else { None })
            .collect();

        utils::print_lines(searched_lines);
    //    match buf_reader {
    //         Ok(mut reader) => {
    //             let mut buf = String::new();
    //             while let Ok(count) = reader.read_line(&mut buf) {
    //                 if count == 0 {
    //                     break;
    //                 }
    //                 self.file_lines.push(buf.clone());
    //                 buf.clear();
    //             }

    //             let searched_lines: Vec<String> = self.file_lines
    //                 .into_iter()
    //                 .filter(|val| val.contains(&pattern) )
    //                 .collect();

    //             utils::print_lines(searched_lines);
    //         },
    //         Err(err) => utils::print_error(&format!("grep_rust: {}\n", err.to_string()))
    //    }

   }

   fn open_to_read<T: Read>(&self, reader: T) -> BufReader<T> {
       let reader = BufReader::new(reader);
       reader
   }
}
