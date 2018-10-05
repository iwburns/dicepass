# dicepass

Generate "dice-ware"-style pass-phrases from the command line.

This application is very much a work-in-progress.  Not intended for
security sensitive applications (for now perhaps).

### Build
```text
$ cargo build --release
```

### Usage
```text
$ cd target/release/
$ ./dicepass
```
will yield a pass-phrase like:
```text
uniformly repeal pushpin shock plasma
```

#### For more info:
```text
$ ./dicepass --help
dicepass 0.1.0
Ian B. <iwburns8@gmail.com>
Generate dice-ware style pass-phrases

USAGE:
    dicepass [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -l, --list <LIST>      Specifies the word-list to use when generating the pass-phrase. [possible values: eff-long,
                           eff-short, eff-short2]
    -w, --words <WORDS>    Specifies the number of words in the generated pass-phrase.
```
