### RUST tooling

In all the cases compiling/checking/building and running programs is done from root folder where _Cargo.toml_ exists

Checking the compiler version 'rustc'
    
    rustc -v </pSath/to/.rs> -o <output-filename> 


Initializing a repository in existing directory

    cargo -v init <name>
    or
    cargo new <name>

To do a sanity check on the code

    cargo check

To build a binary

- DEBUG, file will be in ./target/debug:

        cargo build

- To build and run

        cargo run

- PRODUCTION, file will be in ./target/release:

        cargo build --release

- To build and run

        cargo run --release