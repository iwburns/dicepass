extern crate rand;

use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;
use rand::Rng;
use rand::OsRng;

fn main() {
    let map = get_lines();

    let mut rng = OsRng::new().expect("couldn't get rng");

    let num_words = 6;
    let rolls_per_word = 4;

    let mut words = Vec::new();

    for _ in 0..num_words {
        let mut sequence = String::new();
        for _ in 0..rolls_per_word {
            let roll = rng.gen_range(1u32, 7u32).to_string();
            sequence.push_str(&roll);
        }
        if let Some(value) = map.get(&sequence) {
            words.push(value);
        }
    }

    print_passphrase(words);
}

fn print_passphrase(words: Vec<&String>) {
    for (i, &word) in words.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", word);
    }
}

fn get_lines() -> HashMap<String, String> {
    let mut word_list = File::open("res/wordlist2.txt").expect("couldn't open wordlist file");

    let mut file_contents = String::new();

    word_list
        .read_to_string(&mut file_contents)
        .expect("couldn't read file contents.");

    let line_count = file_contents.as_str().lines().count();

    let map = HashMap::with_capacity(line_count);

    let map = file_contents.lines().fold(map, |mut acc, line| {
        let mut parts = line.split_whitespace();
        let key = parts.next().unwrap_or("").to_string();
        let value = parts.next().unwrap_or("").to_string();
        acc.insert(key, value);
        acc
    });

    return map;
}
