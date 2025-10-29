//////////////////// Day 4 - Functions //////////////



fn main() {
    greet();
    greet_user("Darkstar"); // Function Parameters
    
    let sum = add(5, 1);
    println!("The Sum is : {}", sum);

    outer_function();

    let(sum, product) = calculate(2, 3);
    println!("Sum : {} , Product : {}", sum, product);


    let result = factorial(5);
    println!("Factorial of 5 is :: {}", result);

    let area = calculate_circle(3.5);
    println!("Area of circle with radius 3.5 is : {}", area);
}

fn greet() {
    println!("Hello, Darkstar!");
}



///////  Function Parameters/////

fn greet_user(name: &str) {
    println!("Hello, {}!", name);
}




///////// Return Values from Functions //////

fn add(a: i32, b: i32) -> i32 {
    a + b
}



///////// Nested Functions //////

fn outer_function() {
    fn inner_function() {
        println!("This is an inner function");
    }

    inner_function();
}
/* 
fn inner() → create it (but do nothing)
inner(); → call it (now it executes)
*/



///////// Functions with Multiple Return Values //////

fn calculate(a: i32, b: i32) -> (i32, i32) {
    (a + b, a * b)
}



///////// Function Recursion //////

fn factorial(n: u32) -> u32 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}





/////////////// Hands-On Challenge //////////

// 1. Defines a function to check if a number is even or odd.

fn check_even_odd(num: i32) {

    if num % 2 == 0 {
        println!("The number is 'Even' ");
    } else {
        println!("The number is 'Odd' ");
    }
}

// 2. Includes a function to calculate the area of a circle given its radius.
use std::f64::consts::PI;

fn calculate_circle(radius : f64)  -> f64 {
    PI * radius * radius
}


// 3. Uses a recursive function to find the greatest common divisor (GCD) of two numbers.

fn gcd(a: u32, b: u32) -> (u32) {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}




