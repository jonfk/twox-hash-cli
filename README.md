# TwoX-Hash CLI

A command line interface to the [TwoX-Hash](https://github.com/shepmaster/twox-hash) implementation of the XXHash algorithm.

```
~ twoxhash
twoxhash 1.0.0
Jonathan Fok kan <jfokkan@gmail.com>
Print xxhash checksums

USAGE:
    twoxhash [OPTIONS] [FILE]...

FLAGS:
    -h, --help
            Prints help information

    -V, --version
            Prints version information


OPTIONS:
    -a, --algorithm <ALG>
            Algorithm to be used [default: H64]  [possible values: H64, H32]


ARGS:
    <FILE>...
            Sets the input file to use
```
