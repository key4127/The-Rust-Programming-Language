fn main() {
    let empty_v: Vec<i32> = Vec::new();

    let initial_v = vec![1, 2, 3];

    let mut update_v = Vec::new();

    update_v.push(5);
    update_v.push(6);
    update_v.push(7);
    update_v.push(8);

    // start at 0
    let read_v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &read_v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = read_v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    let panic_v = vec![1, 2, 3, 4, 5];
    
    // panic
    // let does_not_exist = &panic_v[100];
    // not panic
    let does_not_exist = panic_v.get(100);

    // compile error -> mutable and immutable reference
    /*
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    v.push(6);

    println!("The first element is :{first}");
    */

    let immut_v = vec![100, 32, 57];
    for i in &immut_v {
        println!("{i}");
    }

    let mut mut_v = vec![100, 32, 57];
    for i in &mut mut_v {
        *i += 50;
        println!("{}", *i);
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    {
        let v = vec![1, 2, 3, 4];
    }
    // v goes out of scope
}