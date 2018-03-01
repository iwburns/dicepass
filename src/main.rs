extern crate rand;
extern crate clap;

use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

use rand::Rng;
use rand::OsRng;

use clap::{Arg, ArgMatches, App};

fn main() {
    let args = parse_args();

    let file_path = args.value_of("FILE").unwrap();
    let num_words: u32 = args.value_of("WORDS").unwrap().parse().unwrap();
    let rolls_per_word: u32 = args.value_of("ROLLS").unwrap().parse().unwrap();

    let sequence_map = match parse_word_list(file_path) {
        Ok(map) => map,
        Err(e) => panic!(e),
    };

    let words = generate_passphrase(&sequence_map, num_words, rolls_per_word);

    print_passphrase(words);
}

fn generate_passphrase<'a>(
    sequence_map: &'a HashMap<String, String>,
    num_words: u32,
    rolls_per_word: u32,
) -> Vec<&'a String> {
    let mut rng = OsRng::new().expect("couldn't get rng");
    let mut words = Vec::with_capacity(num_words as usize);

    for _ in 0..num_words {
        let mut sequence = String::new();
        for _ in 0..rolls_per_word {
            let roll = rng.gen_range(1u32, 7u32).to_string();
            sequence.push_str(&roll);
        }
        if let Some(value) = sequence_map.get(&sequence) {
            words.push(value);
        }
    }

    words
}

fn print_passphrase(words: Vec<&String>) {
    for (i, &word) in words.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", word);
    }
    println!();
}

fn parse_args<'a>() -> ArgMatches<'a> {
    App::new("dicepass")
        .version("0.1.0")
        .author("Ian B. <iwburns8@gmail.com>")
        .about("Generate dice-ware style pass-phrases")
        .arg(Arg::with_name("FILE")
            .short("f")
            .long("file")
            .help("The input file containing the word-list")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("WORDS")
            .short("w")
            .long("words")
            .help("The number of words in the generated pass-phrase")
            .required(true)
            .takes_value(true)
            .validator(|words| {
                if let Ok(_) = words.parse::<u32>() {
                    return Ok(());
                }
                return Err(String::from("WORDS must be a positive integer"));
            }))
        .arg(Arg::with_name("ROLLS")
            .short("r")
            .long("rolls")
            .help("The number of dice rolls to generate a single word")
            .required(true)
            .takes_value(true)
            .validator(|words| {
                if let Ok(_) = words.parse::<u32>() {
                    return Ok(());
                }
                return Err(String::from("ROLLS must be a positive integer"));
            }))
        .get_matches()
}

fn parse_word_list(file_path: &str) -> std::io::Result<HashMap<String, String>> {
    let mut word_list = File::open(file_path)?;

    let mut file_contents = String::new();
    word_list.read_to_string(&mut file_contents)?;

    let line_count = file_contents.as_str().lines().count();
    let map = HashMap::with_capacity(line_count);

    let map = file_contents.lines().fold(map, |mut acc, line| {
        let mut parts = line.split_whitespace();
        let key = parts.next().unwrap_or("").to_string();
        let value = parts.next().unwrap_or("").to_string();
        acc.insert(key, value);
        acc
    });

    return Ok(map);
}
