use std::io;

fn main() {
    let n:u64;

    loop {
        println!("Please input n.");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Fail to read input");

        let input = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        n = input;

        break;
    }

    let mut f1 = 0;
    let mut f2 = 1;
    let mut f3 = 1;

    for _i in 1..n {
        f3 = f1 + f2;
        f1 = f2;
        f2 = f3;
    }

    println!("The nth Fibonacci number is {f3}.");
}
