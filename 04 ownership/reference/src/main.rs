fn main() {
    let s1 = String::from("hello");
    
    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");

    /*
    let s = String::from("hello");

    change(&s);
    */

    let mut s = String::from("hello");
    // let mut r = & mut s;  only one mutable reference
    // let r = &s; only mutable or immutable

    change(&mut s);

    println!("{s}");

    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{r1} and {r2}");

    let r3 = &mut s;
    println!("{r3}");

    // let reference_to_nothing = _dangle();

    let string = no_dangle();

    println!("{string}");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

/*
fn change(some_string: &String) {
    // immutable
    some_string.push_str(", world");
}
*/

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

/*
fn dangle() -> &String {
    let s = String::from("hello");
    &s
}
*/

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}