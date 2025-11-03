/////// Day 8 - Rust Collections ////////////


///////// VECTORS /////////

use core::num;
use std::collections::{BTreeMap, HashMap, LinkedList};


fn main() {
    let mut numbers = Vec::new();

    numbers.push(1);
    numbers.push(2);
    numbers.push(3);

    println!("Numbers: {:?}", numbers);

    if let Some(last) = numbers.pop() {
        println!("Popped: {}", last);
    }

    println!("Numbers after pop: {:?}", numbers);   
}



//////////// Strings ///////////////

fn main() {
    let mut greeting = String::from("Hello");
    greeting.push_str(", World");
    println!("{}", greeting);

    let substring = &greeting[0..5];
    println!("{}", substring);
}



///////////// Hash Maps ////////////////
use std::collections::HashMap;
fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Alice"), 50);
    scores.insert(String::from("Bob"), 60);
    scores.insert(String::from("Charlie"), 80);

    println!("Scores: {:?}", scores);

    let bob_scores = scores.get("Bob").unwrap();
    println!("Bob's score: {}", bob_scores);
}



//////// BTreeMap /////////
use std::collections::BTreeMap;

fn main() {

    let mut scores = BTreeMap::new();

    scores.insert("Alice", 50);
    scores.insert("Bob", 60);
    scores.insert("Charlie", 70);

    for (name, score) in &scores {
        println!("{}:{}", name, score);
    }
}



///////// VecDeque ////////

use std::collections::VecDeque;

fn main() {
    let mut queue = VecDeque::new();
    queue.push_back(1);
    queue.push_back(2);
    queue.push_front(0);
    
    println!("Queue: {:?}", queue);

    if let Some(front) = queue.pop_front() {
        println!("Removed from front : {}", front);
    }

    println!("Queue after pop: {:?}", queue);
}



////////////// 🔗 LinkedList  /////////
use::std::collections::LinkedList;
fn main() {

    let mut list = LinkedList::new();

    list.push_back(1);
    list.push_back(2);
    list.push_front(0);

    for value in &list {
        println!("{}", value);
    }

    if let Some(front) = list.pop_front() {
        println!("Removed from front: {}", front);
    }

    println!("List after pop: {:?}", list);
}


////// 🎯 Hands-On Challenge //////

/* Create a Rust program that manages a simple inventory system using collections. Implement the following functionalities:

Store items in a vector.
Use a hash map to track the quantities of each item.
Allow adding, removing, and updating item quantities. */


use::std::collections::HashMap;

fn main() {
    let mut inventory = Vec::new();
    let mut quantities = HashMap::new();

    //add items
    inventory.push(String::from("Apples"));
    quantities.insert(String::from("Apples"), 10);

    // update quantity
    *quantities.get_mut("Apples").unwrap() += 5;

    // Remove item
    inventory.retain(|item| item != "Bananas");


    for item in &inventory {
        let quantity = quantities.get(item).unwrap();
        println!("{}:{}", item, quantity);
    }
}
