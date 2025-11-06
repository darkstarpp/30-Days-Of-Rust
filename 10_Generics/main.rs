//////////////// Day 10 - Generics ///////////


fn print_item<T: std::fmt::Debug>(item: T) {
    println!("{:?}", item);
}

fn main() {
    print_item(10); // Works with integers
    print_item(3.14); // Works with floats
    print_item("Hello, Rust!"); // Works with strings

    let data = CrazyStruct { x: 42, y: String::from("Rust"), z: true };

}

struct CrazyStruct<T, U, V> {
    x: T,  // maybe i32
    y: U,  // maybe String
    z: V,  // maybe bool
}



/////////// Creating Generic Functions /////////////

fn largest<T: PartialOrd> (list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}


fn main() {
    let numbers = vec![34,50,25,100,65];
    println!("The largest number is {}", largest(&numbers));

    let chars = vec!['y', 'm','a','q'];
    println!("The largest char is {}", largest(&chars));
}




////////  Structs with Generics  ///////

struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer_point = Point{ x:5, y: 10};
    let float_point = Point{x:2.5, y: 3.3};

    println!("Integer Point: ({}, {})", integer_point.x, integer_point.y);
    println!("Float Point: {}, {}", float_point.x, float_point.y);
}



///////// Generics with Traits /////

fn print_item<T: std::fmt::Display> (item: T) {
    println!("{}", item);
}

fn main() {
    print_item(42);
    print_item("hello");
}

fn print_number<T: std::fmt::Display>(value: T) {
    println!("The number is: {}", value);
}

fn main() {
    print_number(10);
    print_number(4.5);
}



/////////// 📜 Type Constraints and Bounds ///////

fn clone_item<T: Clone> (item: T) {
    item.clone()
// This generic T is allowed only if it has this trait (ability)
}



///// Lifetimes with Generics /////

fn longest<'a> (s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}




///////  Hands-On Challenge ///////

/* Create a Rust program that uses generics to perform the following:

Write a generic function that can accept two values and return the larger of the two.
Create a struct with two generic types and implement a method to display both.
Define an enum that uses generics to store either a number or text. */



// Write a generic function that can accept two values and return the larger of the two.

fn largest<T: PartialOrd> (a: T, b:T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

fn main() {
    let x = 10;
    let y = 20;
    println!("The largest number is : {}", largest(x,y));

    let a = 3.5;
    let b = 5.5;
    println!("The largest number is : {}", largest(a,b));
}




// Create a struct with two generic types and implement a method to display both.

// Define a generic struct `Pair` with two type parameters T and U
// T and U can be any type (like String, i32, f64, etc.)
struct Pair<T, U> {
    first: T,   // first value of type T
    second: U,  // second value of type U
}

// Implement methods for the generic struct Pair<T, U>
impl<T: std::fmt::Debug, U: std::fmt::Debug> Pair<T, U> {
    // `show` is a method attached to Pair
    // &self means the method borrows the instance (does not take ownership)
    fn show(&self) {
        // Print the values of the pair
        // self.first accesses the first field
        // self.second accesses the second field
        println!("Pair contains: ({:?}, {:?})", self.first, self.second);
    }    
}

fn main() {
    // Create an instance of Pair
    // Here T is &str (string slice) and U is i32
    let pair = Pair { first: "Rust", second: 101 };

    // Call the show method on the instance `pair`
    // self in `show` now refers to this `pair`
    // self.first => "Rust", self.second => 101
    pair.show(); // Output: Pair contains: (Rust, 101)
}



// Define an enum that uses generics to store either a number or text. */

enum Option<T> {
    Number(T),
    Text(String),
}

fn main() {
    let num = Option::Number(42);          // Number variant with integer
    let msg: Option<i32> = Option::Text(String::from("Hello Rust!"));

    // Using match to print
    match num {
        Option::Number(val) => println!("Number: {}", val),
        Option::Text(val) => println!("Text: {}", val),
    }

    match msg {
        Option::Number(val) => println!("Number: {}", val),
        Option::Text(val) => println!("Text: {}", val),
    }
}
