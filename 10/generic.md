### 结构体

```Rust
struct Point<T, U> {
    x: T,
    y: U,
}
```

### 枚举

```Rust
enum Option<T> {
    Some(T),
    None,
}   
```

### 函数

```Rust
fn largest<T>(list: &[T]) -> &T
```

### 方法

```Rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
```