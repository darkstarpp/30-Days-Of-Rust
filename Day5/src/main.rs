////////// Ownership & Borrowing /////////


fn main() {
    // ---- Ownership Example ----
    let s1 = String::from("Hello, Rust!");
    println!("{}", s1); 
    let s2 = s1; 
    // println!("{}", s1);  // ❌ Error: s1 moved to s2 (ownership transferred)
    println!("{}", s2);

    // ---- Borrowing & References Example ----
    let s3 = String::from("Rust");
    print_string(&s3);  // ✅ Borrowing s3 by passing a reference (&s3)
    println!("{}", s3); // ✅ Still valid because ownership wasn’t transferred

    let mut s4 = String::from("mutable references");
    mut_refereces(&mut s4);
    println!("{}", s4); // Modified string


    let s5 = String::from("Take Ownership");
    take_ownership(s5); // s5 is moved, and no longer valid here
    // println!("{}", s5);   //this wont work

    let s6 = 5;
    makes_copy(s6);  // s6 is still valid because integers are Copy


        let x = 1;
    let y = String::from("Moving Variable");
    moving_and_copying(x, y); 
    
    let z = String::from("take_ownership of 'Z' ");
    // take_ownership_ex2(z); // The return value is ignored    
    let result = take_ownership_ex2(z); // ✅ The returned String is captured here!
    println!("Ownership successfully transferred back to a new variable: {}", result);
    // The variable 'z' is still invalid, but 'z_owned_back' is now the new owner.

 
    let a = String::from("Using references to avoid unneccessary data copying - Immutable ref");
    print_ref(&a);

    let mut b = String::from("Mutable ref");
    modify_ref(&mut b);  

   
}

fn print_string(s3: &String) {
    // s3 is a reference here — no ownership taken
    println!("{}", s3);
}


fn mut_refereces(s4: &mut String) {
    s4.push_str(" Example");
}



fn take_ownership(s5: String) {
    println!("{}", s5);
}

fn makes_copy(s6: i32) {
    println!("{}", s6);
}




////////////////// Hands-On Challenge ////////////////

/* Write a program that:

Demonstrates moving and copying with variables.
Creates functions that take ownership of their parameters and return a result.
Uses references to avoid unnecessary data copying. */





//Demonstrates moving and copying with variables.

fn moving_and_copying(x: i32, y: String) {
    println!("The value of x is: {} and y is {} ", x, y);
    // x (i32) is copied into the function parameter, y (String) is moved into the function parameter.

}


//Creates functions that take ownership of their parameters and return a result.

fn take_ownership_ex2(z: String) -> String {
    println!("{}", z);
    return z; // Returns ownership back to the caller
}


//Uses references to avoid unnecessary data copying. */


fn print_ref(a: &String) {
    println!("a: {}", a);
}

fn modify_ref(b: &mut String) {
    b.push_str(" - using to avoid unnecessary data copying");
    println!("b: {}", b);
}
