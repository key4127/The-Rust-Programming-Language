use std::io;

fn main() {
    loop {
        println!("Press f to convert Celsius to Fahrenheit.");
        println!("Press c to convert Fahrenheit to Celsius.");
        println!("Press q to quit.");

        let mut opinion = String::new();
        let mut input = String::new();

        io::stdin()
            .read_line(&mut opinion)
            .expect("Fail to read opinion");

        // type not confirmed
        if opinion.trim() == "q" {
            break;
        }

        let mut degree: f64;

        loop {
            println!("Please insert the degree.");

            io::stdin()
                .read_line(&mut input)
                .expect("Fail to read degree");

            let input = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            degree = input;

            break;
        };

        if opinion.trim() == "f" {
            degree = degree * 1.8 + 32.0;
            
            println!("The degrees Fahrenheit is {degree}.");
        }

        if opinion.trim() == "c" {
            degree = ( degree - 32.0 ) / 1.8;

            println!("The degrees Celsius is {degree}.");
        }
    }
}
