// function
/*
cause error: not all types have PartialOrd trait
fn largest<T>(list: &[T]) -> T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
*/

// struct
struct BasicPoint<T> {
    x: T,
    y: T,
}

struct Point<T, U> {
    x: T,
    y: U,
}

// enum
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

// method definitions
struct Point1<T> {
    x: T,
    y: T,
}

// impl for all types
impl<T> Point1<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// impl only for f32
impl Point1<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// generic parameters in struct (X1 Y1) can be different from generic parameters in methods
struct Point2<X1, Y1> {
    x: X1,
    y: Y1,
}

impl <X1, Y1> Point2<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point2<X2, Y2>) -> Point2<X1, Y2> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };

    let p = Point1 { x: 5, y: 10 };
    println!("p.x = {}", p.x());

    let p1 = Point2 { x: 5, y: 10.4 };
    let p2 = Point2 { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}