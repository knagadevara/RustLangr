### Compiling programs in RUST

- Using 'rustc'

        rustc -v </path/to/.rs> -o <output-filename> 

- Using 'cargo'
    - Initializing a repository in existing directory

        cargo -v init <name>

    - Compiling for DEV

        cargo run

    - Compiling for PRD

        cargo build --release  