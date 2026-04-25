# Rust Programming Language - Core Concepts

## 1. **Ownership System**
Rust's most distinctive feature is its ownership system, which manages memory automatically without a garbage collector.

- **Each value has a single owner** at any given time
- **When the owner goes out of scope, the value is dropped**
- **Values can be moved between owners**

```rust
let s1 = String::from("hello");
let s2 = s1; // s1 is moved to s2, s1 is no longer valid
// println!("{}", s1); // This would cause a compile error
```

## 2. **Borrowing and References**
Instead of transferring ownership, you can "borrow" values using references.

- **Immutable references (`&T`)**: Allow reading but not modifying
- **Mutable references (`&mut T`)**: Allow both reading and modifying
- **Borrowing rules**: Either one mutable reference OR any number of immutable references (but not both simultaneously)

```rust
fn calculate_length(s: &String) -> usize {
    s.len() // We can read but not modify
}

fn modify_string(s: &mut String) {
    s.push_str(", world!");
}
```

## 3. **Lifetimes**
Lifetimes ensure that references are valid for as long as needed, preventing dangling pointers.

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

## 4. **Memory Safety Without Garbage Collection**
Rust prevents common memory bugs at compile time:
- **No null pointer dereferences**
- **No buffer overflows**
- **No memory leaks**
- **No data races**

## 5. **Pattern Matching**
Powerful pattern matching with `match` expressions and `if let` constructs.

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}

fn process_message(msg: Message) {
    match msg {
        Message::Quit => println!("Quitting"),
        Message::Move { x, y } => println!("Moving to ({}, {})", x, y),
        Message::Write(text) => println!("Text: {}", text),
    }
}
```

## 6. **Traits**
Traits define shared behavior and enable polymorphism similar to interfaces in other languages.

```rust
trait Display {
    fn fmt(&self) -> String;
}

impl Display for User {
    fn fmt(&self) -> String {
        format!("{}: {}", self.name, self.email)
    }
}
```

## 7. **Error Handling**
Rust uses `Result<T, E>` and `Option<T>` types for explicit error handling, eliminating exceptions.

```rust
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(a / b)
    }
}
```

## 8. **Concurrency and Fearless Parallelism**
Rust's ownership system prevents data races at compile time, making concurrent programming safer.

```rust
use std::thread;

let handle = thread::spawn(|| {
    // Thread-safe code here
    println!("Hello from thread!");
});

handle.join().unwrap();
```

## 9. **Zero-Cost Abstractions**
High-level features don't sacrifice runtime performance - abstractions compile down to efficient low-level code.

## 10. **Strong Static Type System**
- **Type inference** reduces verbosity while maintaining safety
- **Generics** provide code reuse without runtime cost
- **Algebraic data types** (enums and structs) model data precisely

```rust
fn process_items<T>(items: Vec<T>) -> Option<T> 
where 
    T: Clone + PartialEq 
{
    items.first().cloned()
}
```

## 11. **Cargo Package Manager**
Built-in package manager and build system that handles dependencies, testing, and documentation.

## 12. **Immutability by Default**
Variables are immutable unless explicitly marked with `mut`, encouraging safer code patterns.

```rust
let x = 5; // immutable
let mut y = 10; // mutable
y = 15; // OK
// x = 6; // Compile error
```

These concepts work together to make Rust a systems programming language that is both safe and performant, suitable for everything from operating systems to web servers to blockchain applications.