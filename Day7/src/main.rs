////////// Enums ////////

enum Direction {
    North,
    South,
    East,
    West,
}

fn main() {
    let move_direction = Direction::North;
    match move_direction {
        Direction::North => println!("Heading North"),
        Direction::South => println!("Heading South"),
        Direction::East => println!("Heading East"),
        Direction::West => println!("Heading West"),
    }
}



////////// Using Enums with match //////////////

enum TrafficLights{
    Red,
    Yellow, 
    Green,
}

fn action(light: TrafficLights) {
    match light {
        TrafficLights::Green => println!("Go!"),
        TrafficLights::Red => println!("Stop!"),
        TrafficLights::Yellow => println!("Get Ready!"),
    }
}

fn main() {
    let current_light = TrafficLights::Red;
    action(current_light);
}


//////////// Methods on Enums //////////

enum Vehicle {
    Car(String),
    Bike(String),
}

impl Vehicle {
    fn drive(&self) {
        match self {
            Vehicle::Car(name) => println!("Driving a car: {}", name),
            Vehicle::Bike(name) => println!("Riding a bike: {}", name),
        }

    }
}


fn main() {
    let car1 = Vehicle::Car(String::from("sedan"));
    let car2 = Vehicle::Car(String::from("SUV"));
    let car3 = Vehicle::Car(String::from("Truck"));

    let bike1 =  Vehicle::Bike(String::from("Scooter"));
    let bike2 = Vehicle::Bike(String::from("Bullet Bike"));
    let bike3 = Vehicle::Bike(String::from("Cruiser"));

    car1.drive();
    bike3.drive();
}



////////// Enums with Data ///////////

#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

fn main() {
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    println!("Home address: {:?}", home);
    println!("Loopback addrress: {:?}", loopback);
}




/////////////////  Hands-On Challenge ///////////////////

/* Create a Rust program that defines an enum representing a payment method: CreditCard, DebitCard, Cash, and PayPal. 
Write a function to print the payment method used. */


enum PaymentMethod {
    CreditCard(String),
    DebitCard(String),
    Cash,
    Paypal,
}

fn print_payment_method(method: PaymentMethod) {
    match method {
      PaymentMethod::CreditCard(card_number) => println!("Paid with Credit Card: {}", card_number),
      PaymentMethod::DebitCard(card_number) => println!("Paid with Debit Card: {}", card_number),
      PaymentMethod::Cash => println!("Paid with Cash"),
      PaymentMethod::Paypal => println!("Paid with Paypal"),
    }
}

fn main() {

    let payment = PaymentMethod::CreditCard(String::from("1234-5678-9012-3456"));
    print_payment_method(payment);
}