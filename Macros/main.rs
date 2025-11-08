
///////// ⚙ Declarative Macros (macro_rules!) /////////

macro_rules! say_hello {
    () => {
        println!("Hello, Rustaceans!");
    };
}


macro_rules! repeat {
    ($x:expr, $count:expr) => {
        for _ in 0..$count {
            println!("{}", $x);
        }
    };
}

fn main() {
    say_hello!();
    repeat!("Rust is awesome!", 3);
}




///////////////// Procedural Macros ////////////////

/* There are three types of procedural macros:

Derive macros: Used to automatically implement traits for structs or enums.
Attribute macros: Apply attributes to functions, structs, or enums.
Function-like macros: Create custom code generation patterns similar to function calls. */






