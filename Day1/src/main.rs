
fn im_darkstar() {
    let name = "Darkstar";
    let reason = "I'm excited to learn RUST";
    println!("Hello, my name is {} and {}", name, reason);
}



/////////////////////////////////////////////////////
///////////////  Exercises - Day 1 /////////////////
////////////////////////////////////////////////////


/* Exercise: Level 1
Create a new Rust project using Cargo.
Write a program that prints Hello, World! to the console.
Verify that Rust is installed correctly by checking its version. */

fn hello_world() {
    println!("Hello, World!");
}




/* Exercise: Level 2
Modify the main.rs file to declare an immutable variable x with the value of 5.
Change x to be mutable and update its value to 10. */

fn main() {

    // let x = 5;

    let mut x = 5;
    x = 10;
}




/* Exercise: Level 3 (OPTIONAL)
Write a Rust program that declares different data types (integer, float, boolean, string) and prints them.
Experiment with different control flow structures (if, else, for, while). */

fn declare_var() {
    let a = 1;//default
    let b: i32 = -1;
    let c: u32 = 25;
    let d: u64 = 35;
    let e = 5.4; //default

    println!("The values are: a={a}, b={b}, c={c}, d={d}, e={e}");

    let f = false;
    let t: bool = true;
    println!("The values of f and t are: f={f}, t={t}");

    let greet = "Hello, World!";
    println!("The word 'greet' equals to {greet}");
}


fn using_if_else() -> bool {

    let hello = "Hi";
    if hello == "Hi" {
        return true
    }
    else {
        return false
    }
}


fn using_for_loop() {

    //Loop keyword -- Endless loop
    loop {
        println!("Hello, World");
    }

    for i in 0..5 {
        println!("{}", i);
    }
}


fn using_while_loop() {

    let mut i = 0;
    while i < 5 {
        println!("{}", i);
        i+=1;
    }
}