/////////////////////////////////////////////////
///////// Day 2 - Variables and Data Types //////
/////////////////////////////////////////////////


//Data Types
fn main() {

    let repositary_name = "30-Days-Of-Rust";        //string type

    let rating_float = 4.2;                         //float type

    let is_growing_boolean = true;                  //boolean type

    let icon_char = '♥';                            // Unicode character type

    println!("Repository name is: {}", repositary_name);
    println!("Repository rating on 5 is {}", rating_float);
    println!("Repository is growing: {}", is_growing_boolean);
    println!("Repository icon is: {}", icon_char);
}


//Variables and Mutability

fn variable_and_mutability() {
    let x = 5; // Immutable variable
    let mut y = 10; // Mutable variable
    y = 15; // Now this is valid
}

fn mut_ex() {
    let mut mutable_variable = 10;
    println!("Before: {}", mutable_variable);
    mutable_variable += 5;
    println!("After: {}", mutable_variable);
}


//Data Types

fn data_types() {
    let integer = 25;              //integer
    let float = 2.0;               //floating-point
    let boolean = false;          //boolean
    let character = '😂';         //character
    let tuple = (10,9.0);   // tuple
    let array = [1,2,3];      // array
}

fn data_types_() {
    let integer = 34;
    let float = 3.14;
    let is_active = true;
    let char = '😂';
    let name = "Rusty";

    println!(" {} {} {} {} {}", integer, float, is_active, char, name);

}

fn string_ex() {
    // String::from() → Creates an owned, heap-allocated string (mutable if declared 'mut')
    // You can change it later because it's owned by 'my_string'
    let mut my_string = String::from("Hello, Rust!");

    // &str → A string slice, immutable reference to a string literal
    // Stored in read-only memory (static)
    let slice: &str = "Hello, Rust!";

    // You can modify 'my_string' since it's mutable
    my_string.push_str(" Learning Rust!"); 

    println!("Owned String: {}", my_string);
    println!("String Slice: {}", slice);

    my_string.push_str("Love Rust!");
}


fn compound_types() {
    let tuple = (42, 3.14, 'R');
    let array = [1,2,3];
}



/* 🎯 Hands-On Challenge
Create a Rust program that uses variables and demonstrates different data types. Your program should:

Declare and print a variable holding your name.
Create a mutable integer variable representing your current age and update it by adding one.
Use a floating-point variable to store your favorite number and print it.
Include a boolean variable that indicates whether you are learning Rust (set it to true).
Use a character variable to store the first letter of your name. */

fn hands_on_challenge() {
    let name = "Darkstar";
    
    let mut age = 24;
    age += 1;

    let favorite_number = 3.33;

    let is_learning_rust = true;

    let initial = 'P';

    println!("Name: {}", name);
    println!("Updated age: {}", age);
    println!("Favorite Number: {}", favorite_number);
    println!("Learning Rust: {}", is_learning_rust);
    println!("Initial: {}", initial);

}


/////////////////// Exercises - Day 2 //////////////////


/* ✅ Exercise: Level 1

Declare a variable named my_age and set it to your age.
Print the value of my_age to the console.
Create a mutable variable named my_height and assign it your height in centimeters. Update it to a new height.
Declare a variable my_name and assign it your name as a string. Print it to the console.
Create a variable is_student and set it to true if you are a student, or false otherwise. Print the value.
Create a variable birth_year and calculate your birth year by subtracting your age from the current year (you can use a hardcoded current year, e.g., 2024). Print the value. */


fn exercise1() {
    let my_age = 25;
    println!("My age is : {}", my_age);

    let mut my_height = 160;
    my_height = 162; //new height

    let my_name = "Darkstar";
    println!("My Name is : {}" , my_name);

    let is_student = false;
    println!("Is Student: {}", is_student);

    let birth_year = 2025 - 25;
    println!("My Birth Year is : {}", birth_year);

}


/* ✅ Exercise: Level 2

Create variables for each numeric type (integer and float) and print their values:
An integer variable my_integer set to any integer value.
A floating-point variable my_float set to any float value.
Declare a boolean variable is_learning_rust and set it to true. Print the value.
Create a character variable favorite_letter and assign it your favorite letter. Print it.
Create an array of integers called my_scores that holds your last five test scores. Print the entire array.
Create a string variable hobby and assign it one of your hobbies. Print it, and then concatenate it with another string to create a sentence (e.g., "I enjoy [hobby]!"). Print the complete sentence. */


fn exercise2() {
    let my_integer = -1;
    let my_float = 3.33;

    let is_learning_rust = true;
    println!("Learning Rust: {}", is_learning_rust);

    let favorite_letter = 'P';
    println!("My Favorite Letter is : {}", favorite_letter);
    
    let my_scores = [99,99,91,91,70];
    println!( "My last five test scores are: {:?}", my_scores);

    let mut hobby = String::from("Coding in Rust");
    println!("My Hobby is : {}", hobby);
    hobby.push_str(" and I enjoy it!");
    println!("{}", hobby);

}
