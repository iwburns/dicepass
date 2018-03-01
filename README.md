#dicepass

Generate "dice-ware"-style pass-phrases from the command line.

This application is very much a work-in-progress.  Not intended for
security sensitive applications (for now perhaps).

###Build
```text
$ cargo build --release
```

###Usage
```text
$ cd target/release/
$ dicepass -f path/to/wordlist.txt -w 6 -r 4
```
will yield a pass-phrase like:
```text
bonnet siesta effects ebook fiddle molecule
```

####For more info:
```text
$ dicepass --help
dicepass 0.1.0
Ian B. <iwburns8@gmail.com>
Generate dice-ware style pass-phrases

USAGE:
    dicepass --file <FILE> --rolls <ROLLS> --words <WORDS>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --file <FILE>      The input file containing the word-list
    -r, --rolls <ROLLS>    The number of dice rolls to generate a single word
    -w, --words <WORDS>    The number of words in the generated pass-phrase
```
