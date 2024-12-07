enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}
enum IpAddr1 {
    V4(String),
    V6(String),
}

enum IpAddr2 {
    V4(u32, u32, u32, u32), //structs, enum, etc. can also be included
    V6(String),
}

fn route(ip_kind: IpAddrKind) {}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {

    }
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let home = IpAddr1::V4(String::from("127.0.0.1"));

    let loopback = IpAddr1::V6(String::from("::1"));

    let home = IpAddr2::V4(127, 0, 0, 1);

    let loopback = IpAddr2::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5);
    let some_char = Some('e');
    
    let some_absent: Option<i32> = None;

    // convert Option<T> to T before use it
}
