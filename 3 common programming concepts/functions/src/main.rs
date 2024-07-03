fn main() {

    // functions
    println!("Hello, world!");

    another_function();

    // parameters
    function_with_parameter(5);

    print_labeled_measurement(5, 'h');

    // statements and expressions
    // the scope block is an expression
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

    // functions with return values
    let x = five();

    println!("The value of x is: {x}");

    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn another_function() {
    println!("Another function.");
}

fn function_with_parameter(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}