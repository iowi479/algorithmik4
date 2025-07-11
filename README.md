# Assignment 4

## Studentinformation
Simon Schindler
3538650
st173013@stud.uni-stuttgart.de

## Dependencies
- [Rust](https://www.rust-lang.org/tools/install): I used the latest stable version 1.86.0 but it should be backwards compatible for many Versions since no brand-new features where used.

## Running
Either run cargo build to build both needed binaries by running: `cargo build -r`
OR just run `make` which will run them in sequence like it was asked for on the assignment.

For the binary, there are two arguments to be passed:

- First argument: The path to the input file
In the makefile this is `./dewiki-20220201-clean.txt` by default

- Second argument: The amount of articles to process
In the makefile this is `100000` by default

For other variants just run
```bash
cargo run --release --bin problem1 <inputfile> <amount>
```


## Important
When running into memory issues, try to reduce the amount of threads to be used by the parallel sorting.
You can do this by setting the `RAYON_NUM_THREADS` environment variable to a lower value.
```bash
RAYON_NUM_THREADS=4 cargo run --release --bin problem1 <inputfile> <amount>
```


