//////////////  Structs //////////////

struct User {
    username : String,
    email : String,
    active : bool,
    sign_in_count : u64,
}


//Creating Instances of Structs

fn main() {
    let user1 = User {
        username: String::from("Rustacean"),
        email: String::from("rust@example.com"),
        active : true,
        sign_in_count : 1,
    };

    println!("Username: {}", user1.username);

    let user2 = create_user(String::from("rustacean@rust.com"), String::from("Rustacean"));
    println!("Username: {}", user2.username);


    //Updating Structs
    let user3 = User {
        email: String::from("new@example.com"),
        ..user1
    };
    println!("Username: {}", user3.username);



}



// Using Structs in Functions
fn create_user(email: String, username: String) -> User {
    User{
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}



//Tuple Structs
struct Color (i32, i32, i32);

fn main() {
    let black = Color(0,0,0);
    println!("Color: {}, {}, {}", black.0, black.1, black.2);
}



//Unit-Like Structs

struct AlwaysEqual ;

fn main() {
    let_subject = AlwaysEqual;
}


// Structs with Methods

struct  Rectangle {
    width : u32,
    height : u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect = Rectangle { width: 30, height: 50};
    println!("Area: {}", rect.area());
}



//////////// Hands-On Challenge ///////

/* Create a program that defines and uses a struct for a book with fields like title, author, pages, and publisher.
Write functions to calculate and display book details using the struct. */

struct Book {
    title : String,
    author : String,
    pages : u32,
    publisher : String,
}

impl Book {

    fn display_book_details(&self) {
        println!("The book details : title:{}, author:{}, pages: {}, publisher: {}", self.title, self.author, self.pages, self.publisher);
    }

    fn calculate(&self) {
        if self.pages > 200 {
            println!("It will take more than a day to finish the book");
        } else if self.pages > 100 {
            println!("It will take 10 hours to finish the book");            
        } else {
            println!("It will 5 hours to finish the book");
        }
    }    
}


fn main() {
    let display_details = Book { title: String::from("Rust in 30 days"), author: String::from("Darkstar"), pages : 250, publisher: String::from("Darkstar Publishing") };
    display_details.display_book_details();
    display_details.calculate();
}