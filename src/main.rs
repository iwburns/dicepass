extern crate clap;
extern crate lib_dicepass_gen;

use clap::{Arg, ArgMatches, App};
use lib_dicepass_gen::*;

fn main() {
    let args = parse_args();

//    let file_path = args.value_of("FILE").unwrap();
//    let rolls_per_word: u32 = args.value_of("ROLLS").unwrap().parse().unwrap();

    let word_count = if let Some(num_words) = args.value_of("WORDS") {
        WordCount::from(num_words.parse().ok())
    } else {
        WordCount::Default
    };

    let pass_config = PassGenConfig::from_eff_long(word_count);

    println!("{}", generate(pass_config));
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
