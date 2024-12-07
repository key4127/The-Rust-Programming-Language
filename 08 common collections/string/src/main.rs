fn main() {
    let mut s = String::new();

    let data = "initial contents";
    
    let s = data.to_string();

    let s = "initial contents".to_string();

    let s = String::from("initial contents");

    let s = String::from("你好");
    println!("{s}");

    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{s}");

    let mut s1 = String::from("foo");
    let s2 = "bar"; // this is &str, String::from("bar") is String and will cause error
    s1.push_str(s2);
    println!("s1 is {s1}");
    println!("s2 is {s2}");

    let mut s = String::from("lo");
    s.push('l');
    println!("{s}");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world");
    let s3 = s1 + &s2; // s1 can no longer be used
    println!("s2 is {s2}");
    println!("s3 is {s3}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("{s}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}"); // does not take ownership
    println!("s1 is {s1}");
    println!("s2 is {s2}");
    println!("s3 is {s3}");
    println!("s is {s}");

    let hello = "Здравствуйте"; // a character is two (or other quantities) bytes long
    let s = &hello[0..4];
    println!("{s}"); // Зд

    for c in "Зд".chars() {
        println!("{c}");
    }

    for c in "Зд".bytes() {
        println!("{c}");
    }
}
