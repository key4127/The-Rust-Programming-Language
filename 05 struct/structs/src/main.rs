struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_account: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

fn main() {
    
    //struct

    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_account: 1,
    };

    if user1.active == true {
        println!("The username is {}.", user1.username);
        println!("The email is {}.", user1.email);
        println!("The sign in account is {}.", user1.sign_in_account);
    }
    
    user1.email = String::from("anotheremail@example.com");

    println!("The new email is {}.\n", user1.email);

    let user2 = build_user(String::from("email"), String::from("username"));

    if user2.active == true {
        println!("The username is {}.", user2.username);
        println!("The email is {}.", user2.email);
        println!("The sign in account is {}.\n", user2.sign_in_account);
    }

    let user3 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    //the username of user1 has been moved.

    if user3.active == true {
        println!("The username is {}.", user3.username);
        println!("The email is {}.", user3.email);
        println!("The sign in account is {}.", user3.sign_in_account);
    }

    //other structs

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_account: 1,
    }
}