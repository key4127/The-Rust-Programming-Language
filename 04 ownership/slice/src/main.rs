fn main() {
    let mut s = String::from("hello world");

    let index = _first_word(&s);

    println!("the index is: {index}");

    s.clear();
    // index is still 5 but meaningless

    let s = String::from("hello world");

    let word = first_word(&s);

    // cause error
    // s.clear();

    println!("the word is: {word}");

    // all work
    /*
    let my_string = String::from("hello world");

    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);

    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    let word = first_word(my_string_literal);
    */
}

fn _first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

//fn first_word(s: &String) -> &str

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]
        }
    }

    &s[..]
}