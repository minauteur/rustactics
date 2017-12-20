//!Utilities Module
//!Various Helper functions and error definitions used throughout the project live here

use std::io::{BufReader, BufWriter, Read, Write};
// use std::fmt::Write;
use std::io::BufRead;
use std::fs::File;
use std::io;
use std::path::PathBuf;


static LOC_SEED_DIR: &'static str = "shakespeare.txt";

pub fn format_txt() {
    let p = PathBuf::from(&LOC_SEED_DIR);
    let f = File::open(&p).unwrap();
    let file = BufReader::new(&f);
    let mut writer = BufWriter::new(io::stdout());
    for (num, line) in file.lines().enumerate() {
        let l = line.unwrap();
        let mut b = [0;4];
        let mut d = [0,4];
            let mut chars: String = l.chars().collect();
            for c in chars.clone().chars().into_iter() {
                let p: char = '.';
                let r: char = '\n';
                match &c {
                    p => { 
                        if c == *p {
                            let index = chars.find('.').unwrap_or_default();
                            chars.insert(index, '\n' as char);
                        }
                        else {continue}
                    },
                    r => {
                        let index = chars.find('\n').unwrap_or_default();
                        chars.remove(index);
                    },
                    _ => (),
                }
            }
            writeln!(writer, "{}", chars).unwrap();
        // if num % 4 == 1 {
        //     writeln!(writer, "{}", l).unwrap();
        // }
    }
}

pub fn read_file() {
    let path = PathBuf::from(&LOC_SEED_DIR);
    let txt_src = File::open(&path).unwrap();
    let txt_dest = File::create("output.txt").expect("Couldn't create destination file for output!");
    let reader = BufReader::new(&txt_src);
    let mut writer = BufWriter::new(&txt_dest);
    for (num, l) in reader.lines().enumerate() {
        let line = l.unwrap();
        let mut current_ln: String = line.chars().collect();
        current_ln.trim_left();
        let at_newline = current_ln.trim_right().len();
        current_ln.truncate(at_newline);
        let line_vec = &current_ln[..];
        let content = line_vec.split_whitespace().collect::<Vec<_>>();
        write!(writer, "{:?}", content);
        }
     
    }
