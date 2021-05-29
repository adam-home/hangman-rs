extern crate rand;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

use self::rand::distributions::{IndependentSample, Range};

fn valid_word(word: &String) -> bool {
    word.chars().all(|c| c.is_lowercase()) && word.len() >= 4
}

fn random_word_from_dict() -> String {
    let reader = BufReader::new(File::open("/usr/share/dict/words").unwrap());

    let count = reader.lines().count();
    
    let mut rng = rand::thread_rng();
    let rnd = Range::new(0, count-1).ind_sample(&mut rng);

    // Reset to start of file
    let mut reader = BufReader::new(File::open("/usr/share/dict/words").unwrap());
    let mut lines_read = 0;
    let mut line = String::new();
    let mut len = 0;
    while lines_read < rnd {
        lines_read += 1;
        line.clear();
        len = reader.read_line(&mut line).unwrap()
    }

    line.truncate(len - 1); // Remove \n
    line
}

pub fn choose_word() -> String {
    let mut word;
    loop {
        word = random_word_from_dict();
        if valid_word(&word) {
            break;
        }
    }
    word
}
