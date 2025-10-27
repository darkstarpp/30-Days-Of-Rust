///Day 3 - Control Flow

// if Statement
fn main() {
    let number = 10;
    if number > 5 {
        println!("The number is greater than 5");
    }

    // call the other functions
    if_else_elseif();
    else_and_elseif();
    loop_statement();
    while_loop();
    for_loop();
    match_match();
    check_even_odd();
    hands_on_challenge2();
    hands_on_challenge3();
}

fn if_else_elseif() {
    let marks = 85;

    if marks >= 90 {
        println!("Grade: A");
    } else if marks >= 75 {
        println!("Grade: B");
    } else if marks >= 60 {
        println!("Grade: C");
    } else {
        println!("Grade: F");
    }
}



// else and else if Statements

fn else_and_elseif() {
    let age = 18;
    if age >= 21 {
        println!("You can Drink alcohol");
    } else if age >= 18 {
        println!("You are an adult, but cannot drink alcohol");
    } else {
        println!("You are a minor.");
    }
}



// loop Statement

fn loop_statement() {

    let mut count = 0;

    loop {
        count += 1;
        if count == 10 {
            println!("Breaking the loop at count: {}", count);
            break;
        }
    }

}



// while Loop 

fn while_loop() {

    let mut num = 1;

    while num <= 5 {
        println!("Loop count: {}", num);
        num += 1;
    }
}




// for Loop

fn for_loop() {    

    for num in 1..4 {
        println!("Num: {}", num);
    }
}




// Control Flow with match

fn match_match() {
    let traffic_light = "green";

    match traffic_light {
        "green" => println!("Go"),
        "yellow" => println!("Slow down"),
        "red" => println!("Stop"),
        _  => println!("Invalid color"),        
    }
}



////////////  Hands-On Challenge ////////////

/* Write a program that:

Asks the user to input a number.
Uses an if statement to check if the number is even or odd */

use std::io;

fn check_even_odd() {
    println!("Enter a number:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let num: i32 = input.trim().parse().unwrap();

    if num % 2 == 0 {
        println!("The Number is Even : {}", num);
    } else {
        println!("The number is Odd: {}", num);
    }
}


// 3. Use a loop to print numbers from 1 to 5
fn hands_on_challenge2() {
    let mut number = 1;

    loop {
        println!("Number: {}", number);

        if number == 5 {
            break;
        }

        number += 1;
    }
}


// 4. Implement a match statement to respond to different days of the week, 
// e.g., "Monday" => "Start of the week!", "Friday" => "Weekend is coming!", etc.

fn hands_on_challenge3() {
    let start_day = "Monday";

    match start_day {
        "Monday" => println!("Start of the week!"),
        "Tuesday" => println!("2nd work day"),
        "Wednesday" => println!("3rd day"),
        "Thursday" => println!("4th day"),
        "Friday" => println!("Weekend is coming!"),
        _ => println!("It's the weekend!"),
    }
}
