extern crate clap;
extern crate lib_dicepass_gen;

//use std::io::prelude::*;
//use std::fs::File;
//use std::collections::HashMap;

use clap::{Arg, ArgMatches, App};
use lib_dicepass_gen::PassphraseGenerator;

fn main() {
    let args = parse_args();

//    let file_path = args.value_of("FILE").unwrap();
//    let rolls_per_word: u32 = args.value_of("ROLLS").unwrap().parse().unwrap();

    let pg = PassphraseGenerator::new();
    let passphrase;

    if let Some(num_words) = args.value_of("WORDS") {
        passphrase = pg.generate_with_length(num_words.parse().unwrap());
    } else {
        passphrase = pg.generate();
    }

    println!("{}", passphrase);
}

fn parse_args<'a>() -> ArgMatches<'a> {
    App::new("dicepass")
        .version("0.1.0")
        .author("Ian B. <iwburns8@gmail.com>")
        .about("Generate dice-ware style pass-phrases")
        .arg(Arg::with_name("FILE")
            .short("f")
            .long("file")
            .help("Specifies the input file containing the word-list.")
            .required(false)
            .takes_value(true))
        .arg(Arg::with_name("WORDS")
            .short("w")
            .long("words")
            .help("Specifies the number of words in the generated pass-phrase.")
            .required(false)
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
            .help("Specifies the number of dice rolls to generate a single word.  This must match the format of the word-list file.")
            .required(false)
            .takes_value(true)
            .validator(|words| {
                if let Ok(_) = words.parse::<u32>() {
                    return Ok(());
                }
                return Err(String::from("ROLLS must be a positive integer"));
            }))
        .get_matches()
}
//
//fn parse_word_list(file_path: &str) -> std::io::Result<HashMap<String, String>> {
//    let mut word_list = File::open(file_path)?;
//
//    let mut file_contents = String::new();
//    word_list.read_to_string(&mut file_contents)?;
//
//    let line_count = file_contents.as_str().lines().count();
//    let map = HashMap::with_capacity(line_count);
//
//    let map = file_contents.lines().fold(map, |mut acc, line| {
//        let mut parts = line.split_whitespace();
//        let key = parts.next().unwrap_or("").to_string();
//        let value = parts.next().unwrap_or("").to_string();
//        acc.insert(key, value);
//        acc
//    });
//
//    return Ok(map);
//}
