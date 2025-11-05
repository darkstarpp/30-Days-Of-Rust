
///////////////// Day 12 - Modules and Crates ///////////////


// Declare the module `vehicles`
mod vehicles {
    // Functions inside the module
    pub fn car_info() {
        println!("Car: Tesla Model S");
    }

    pub fn ship_info() {
        println!("Ship: Titanic");
    }

    pub fn train_info() {
        println!("Train: Bullet Train");
    }
}

fn main() {
    // Calling functions from the module using :: syntax
    vehicles::car_info();
    vehicles::ship_info();
    vehicles::train_info();
}


/////////// Nested Modules  ////////////

mod outer {
    pub mod inner {
        pub fn greet() {
            println!("Hello from inner module!");
        }
    }
}

fn main() {
    outer::inner::greet();
}



////////////// Crates ////////////

//Creating a Crate -- cargo new my_crate --lib

use serde::Serialize;

#[derive(Serialize)]
struct MyStruct {
    name: String,
    age: u32,
}


mod my_utils {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}

fn main() {
    let result = my_utils::add(5, 7);
    println!("The result is: {}", result);
}



///////// Hands-On Challenge /////

/* Create a Rust program that demonstrates the use of modules and crates. Your program should:

Create a module named math_operations that contains two public functions: add and subtract.
In the main function, call these functions and print their results.
Create a new library crate named geometry with a function that calculates the area of a rectangle.
Use your geometry crate in another binary crate to calculate and print the area. */


mod math_operations {
     pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn subtract(a: i32, b: i32) -> i32 {
        a - b
    }
}


fn main() {
    let sum = math_operations::add(5, 3);
    let difference = math_operations::subtract(5, 3);

    println!("Sum: {}", sum);
    println!("Difference: {}", difference);

    let area = geometry::rectangle_area(10.0, 5.0);
    println!("Area of rectangle: {}", area);
}