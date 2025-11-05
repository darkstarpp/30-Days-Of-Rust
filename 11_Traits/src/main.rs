
////////// Defining Traits ///////

trait Greet {
    fn greet(&self) -> String;
}

struct Person {
    name: String,
}

impl Greet for Person {
    fn greet(&self) -> String {
        format!("Hello, my name is {}", self.name)
    }
}

fn main() {
    let person = Person { name: String::from("Alice") };
    
    // This prints the string returned by greet()
    println!("{}", person.greet());
}



///////// Implementing a Trait for a Struct //////

trait Describe {
    fn description(&self) -> String ;
}

struct  Person {
    name: String,
    age: u32,
}

impl  Describe for Person {
    fn description(&self) -> String {
        format!("{} is {} years old", self.name, self.age)
    }
}

fn main() {
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };

    println!("{}", person.description()); // Outputs: Alice is 30 years old
}




///// Common Traits in Rust /////

use std::fmt::format;

#[derive(Debug, Clone)]

struct Item {
    name : String,
    value : i32,

}


fn main() {
    let item1 = Item {
        name : String::from("Rust Book"),
        value: 50,
    };

    let item2 = item1.clone();

    println!("{:?}", item1);
    println!("{:?}", item2);
}



//////  Traits with Generics ///////

struct  Container<T> {
    value: T,
}

impl<T> Container<T> {
    fn new(value: T) -> Self {
        Container { value }
    }
}



//////// Trait Bounds ////////

fn print_greet<T: Greet> (item: T) {
    println!("{}", item.greet());
}  // <T: Greet> Person(struct) implements Greet(trait)




////////// 🎯 Hands-On Challenge /////////////
/// 
/* Create a Rust program that:

Defines a trait named Describe with a method describe that returns a string.
Implements the trait for two different structs: Car and Bike.
Uses a generic function to print the description of any item that implements Describe. */

trait Describe { 
    fn describe(&self) -> String;
}

struct Car {
    brand: String,
    year: i32,
}

struct Bike {
    model: String,
    color: String,
}

impl Describe for Car {
    fn describe(&self) -> String {
        format!("Car: {} ({})", self.brand, self.year)
    }
}

impl Describe for Bike {
    //T in print_description<T: Describe> is any type that implements the Describe trait
    fn describe(&self) -> String {
        format!("Bike: {} ({})", self.model, self.color)
    } 
}



fn print_description<T: Describe> (item: &T) {
    println!("{}", item.describe());
}


fn main() {
    let car = Car { brand: "Toyota".to_string(), year: 2020 };
    let bike = Bike { model: "Ducati".to_string(), color: "Red".to_string() };

    print_description(&car);
    print_description(&bike);
}
