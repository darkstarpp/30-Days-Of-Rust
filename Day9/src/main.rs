fn main() {
    let v = vec![1, 2, 3];

    // This will panic because there is no element at index 99
    println!("Value at index 99: {}", v[99]);
}



///// Option & Unwrap ////////

fn get_value(index: usize) -> Option<i32> {
    let values = vec![10,20,30] ;
    values.get(index).copied() //return Some(value) or None
} 

fn main() {
    let value = get_value(5).unwrap_or(0); //Falback value if index is out of bounds
    println!("Value: {}", value);
}



/// another example
fn get_value(index: usize) -> Option<i32> {
    let values = vec![10,20,30];
    values.get(index).copied() // Return Some(Value) or None
}

fn main() {
    match  get_value(1) { 
        Some(value) => println!("Value : {}", value),
        None => println!("No value found at that index."),   
    }
}



//////// Result Type //////////

use std::io::{self, Read};
use std::fs::File;

fn read_file(filename: &str) -> Result<String, io::Error> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    match read_file("hello.txt") {
        Ok(contents) => println!("File contents:\n{}", contents),
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}



//////// Handling Multiple Error Types ////

use std::fmt;
use std::fs::File;
use std::io::{self, Read};

#[derive(Debug)]
enum MyError {
    Io(io::Error),
    parse(std::num::ParseIntError),
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyError::Io(e) => write!(f, "I/O error: {}", e),
            MyError::parse(e) => write!(f, "Parse error: {}", e),
        }
    }
}

fn read_file(filename: &str) -> Result<String, MyError> {
    let mut file = File::open(filename).map_err(MyError::Io)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(MyError::Io)?;
    Ok(contents)
}



fn main() {
    match read_file("hello.txt") {
        Ok(contents) => println!("File contents:\n{}", contents),
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}



///////  Iterating Over Results /////


fn parse_numbers(input: Vec<&str>) -> Vec<Result<i32, std::num::ParseIntError>> {
    input.iter().map(|s|s.parse::<i32>()).collect()
}

fn main() {
    let inputs = vec!["42", "93", "hello"];
    let results: Vec<_> = parse_numbers(inputs);

    for result in results {
        match result {
            Ok(num) => println!("Parsed number: {}", num),
            Err(e) => eprintln!("Error parsing input: {}", e),
        }
    }
}


/////// Result Type //////// 

use std::fs::File;
use std::io::{self, Read};

fn read_file(filename: &str) -> Result<String, io::Error> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    match read_file("hello.txt") {
        Ok(contents) => println!("File contents:\n{}", contents),
        Err(e) => eprintln!("Error reading file: {}", e), // Using eprintln! for error output
    }
}


//////  Custom Error Types /////////

use std::fmt;

#[derive(Debug)]
enum MyError {
    NotFound,
    InvalidInput,
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &

mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyError::NotFound => write!(f, "Resource not found"),
            MyError::InvalidInput => write!(f, "Invalid input provided"),
        }
    }
}

fn perform_action() -> Result<(), MyError> {
    Err(MyError::InvalidInput)
}

fn main() {
    match perform_action() {
        Ok(_) => println!("Action performed successfully!"),
        Err(e) => eprintln!("Error: {}", e),
    }
}


//////////// Hands-On Challenge ///////////









