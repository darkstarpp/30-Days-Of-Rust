///////// 📖 Writing Tests /////////

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}


//// Unit Tests

fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_ad() {
        assert_eq!(add(2,3), 5);
        assert_eq!(add(0,0), 0);
        assert_eq!(add(-1,1), 0);
    }

    
}



/////////// Handling Failures ////////
#[test]
fn it_fails() {
    assert!(false); // This will panic!
}


//////// The should_panic Attribute //////

#[test]
#[should_panic]
fn it_panics() {
    assert_eq!("Hello", "world");
}

#[test]
#[should_panic(expected = "assertion `left == right` failed")]
fn it_panics_with_message() {
    assert_eq!("Hello", "world");
}




///////// Integration Tests /////////////

