fn main() {
    // string
    let mut s = String::from("hello");

    s.push_str(", world!");

    println!("{s}");


    // move and clone
    let s1 = String::from("hello");
    // s1 has been dropped
    let s2 = s1;

    println!("{s2}, world!");

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");


    // ownership and functions
    let s = String::from("hello");

    // s was moved and invalid here
    takes_ownership(s);

    // println!("{s}"); will cause an error

    let x = 5;

    makes_copy(x);


    // return values
    let s1 = gives_ownership();

    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2);

    println!("s1 = {s1}");

    // println!("s2 = {s2}"); cause error
    println!("s2 is invalid");

    println!("s3 = {s3}");

    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{s2}' is {len}");
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
}

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}