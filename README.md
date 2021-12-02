# Usage

```
USAGE:
advent2021 [OPTIONS] --day <day> <SUBCOMMAND>

FLAGS:
-h, --help       Prints help information
-V, --version    Prints version information

OPTIONS:
-d, --day <day>        Day number
-f, --file <file>      Input filename
-v, --value <value>    Input value

SUBCOMMANDS:
help     Prints this message or the help of the given subcommand(s)
part1    Get 1st solution
part2    Get 2nd solution
```

```shell
$ advent2021 --day=1 --file=inputs/day1_demo.txt part1
$ # or cargo run --release -- -d 1 -f inputs/day1_demo.txt part2
```