use Testing::add;

#[test]

fn test_add_function() {
    assert_eq!(add(2,3),5);
}


#[cfg(test)]

mod tests {
    #[test]
    fn test_integration() {
        let result = super::add(2, 3); // Import the function to test
        assert_eq!(result, 5);
    }
}

extern crate adder;

#[test]
fn it_works() {
    assert_eq!(4, adder::add_two(2));
}





///////////// 📝 Documentation Tests ///////

/// Adds two numbers.
///
/// # Examples
///
/// ```
/// let result = add(2, 3);
/// assert_eq!(result, 5);
/// ```
fn add(a: i32, b: i32) -> i32 {
    a + b
}


/// This function adds two to its argument.
///
/// # Examples
///
/// ```
/// assert_eq!(4, adder::add_two(2));
/// ```
pub fn add_two(a: i32) -> i32 {
    a + 2
}



////////// ⚙️ Concurrency in Testing ///////

// RUST_TEST_THREADS=1 cargo test





////////// 🎯 Hands-On Challenge /////////

/* Create a Rust program that implements functions with unit tests and integration tests. Your program should:

Implement a function that subtracts two numbers.
Write unit tests for the subtraction function.
Create an integration test in a separate file to test the subtraction function.
Include a documentation test in the function’s comments.
 */


fn subtraction(a: i32, b: i32) -> i32 {
    a - b
}


#[test]
fn test_sub() {
    assert_eq!(subtraction(6, 7), -1);
    assert_eq!(subtraction(6, 5), 1);
    assert_eq!(subtraction(8, 1), 7);
}


#[cfg(test)]
mod tests {
    use Testing::subtraction;

    #[test]
    fn test_integration() {
        let result = subtraction(10, 4); // Import the function to test
        assert_eq!(result, 6);
    }
}



// // Include a documentation test in the function’s comments.

// // Ans in Src/lib.rs
// /// Subtracts the second number from the first.
// ///
// /// # Examples
// ///
// /// ```
// /// use Testing::subtraction;
// ///
// /// let result = subtraction(10, 4);
// /// assert_eq!(result, 6);
// /// ```
// pub fn subtraction(a: i32, b: i32) -> i32 {
//     a - b
// }
