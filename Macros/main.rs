
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


////////  Attribute Macros //////

#[proc_macro_attribute]
pub fn my_attribute_macro(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Your code transformation logic here
}



////////// Derive Macros ///////

#[derive(Debug)]
struct Book {
    title: String,
    author: String,
}

fn main() {
    let book = Book { title: String::from("Rust Book"), author: String::from("John Doe") };
    println!("{:?}", book);
}



// Custom derive macros allow you to automatically implement traits for your structs or enums. For example, you can derive serialization traits like this

#[derive(Debug, Serialize, Deserialize)]
struct MyStruct {
    name: String,
    age: u8,
}

// Serialize lets you convert the struct → JSON (or other formats).
// Deserialize lets you convert JSON → struct.






///////////// Function-like Macros /////


use proc_macro::TokenStream;

#[proc_macro] // tells compiler this is a procedural macro
pub fn make_hello(_input: TokenStream) -> TokenStream {
    "fn hello() { println!(\"Hello!\"); }".parse().unwrap()
}

use my_macro_lib::make_hello;

make_hello!(); // expands → fn hello() { println!("Hello!"); }

fn main() {
    hello(); // runs the generated code
}





