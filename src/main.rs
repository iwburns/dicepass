extern crate clap;
extern crate lib_dicepass_gen;

use clap::{Arg, ArgMatches, App};
use lib_dicepass_gen::*;

fn main() {
    let args = parse_args();

    let word_count = if let Some(num_words) = args.value_of("WORDS") {
        WordCount::from(num_words.parse().ok())
    } else {
        WordCount::Default
    };

    let pass_config = match args.value_of("LIST").unwrap_or("eff-long") {
        "eff-long" => PassGenConfig::from_eff_long(word_count),
        "eff-short" => PassGenConfig::from_eff_short(word_count),
        "eff-short2" => PassGenConfig::from_eff_short_2(word_count),
        _ => unreachable!(),
    };

    println!("{}", generate(pass_config));
}

fn parse_args<'a>() -> ArgMatches<'a> {
    App::new("dicepass")
        .version("0.1.0")
        .author("Ian B. <iwburns8@gmail.com>")
        .about("Generate dice-ware style pass-phrases")
        .arg(Arg::with_name("LIST")
            .short("l")
            .long("list")
            .help("Specifies the word-list to use when generating the pass-phrase.")
            .required(false)
            .takes_value(true)
            .possible_values(&["eff-long", "eff-short", "eff-short2"]))
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
        .get_matches()
}
