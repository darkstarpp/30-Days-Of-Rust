fn main() {
    println!("Hello, world!");
}


// Cargo: Rust’s official package manager and build system, manages projects, dependencies, builds, tests.

// cargo --version: Checks Cargo installation and version.

// cargo new <project>: Creates a new Rust project with a src folder and Cargo.toml.

// Cargo.toml: Project metadata file, lists package info, dependencies, and optional features.

// [package]: Section in Cargo.toml with name, version, edition.

// [dependencies]: Section in Cargo.toml to declare external crates your project needs.

// cargo build: Compiles the project into target/debug (or --release for optimized).

// cargo run: Compiles (if needed) and executes the project.

// cargo test: Runs unit tests and integration tests.

// cargo doc: Generates documentation for your crate.

// cargo publish: Publishes your crate to crates.io.

// cargo add <crate>: Adds a dependency to Cargo.toml and downloads it automatically.

// cargo update: Updates dependencies to the latest compatible versions.

// Dependencies syntax: "crate_name" = "version" or "crate_name" = "^version" for ranges.

// Project structure: my_project/ → Cargo.toml + src/main.rs (binary) or src/lib.rs (library).

// Crates.io: Official Rust package registry for publishing or searching crates.

// cargo search <keyword>: Searches crates.io for a specific crate or keyword.

// Features in Cargo.toml: Optional functionality users can enable in a crate.

// Edition field: Rust language edition affecting syntax/features (2018, 2021).