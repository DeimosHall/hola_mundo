//! A binary crate to test an upload to crates.io

fn main() {
    say_hello();
}

/// The classic Hello World but in Spanish
/// # Example
/// ```
/// say_hello();
/// assert_eq!("Hola Mundo :D", say_hello())
/// ```
/// Output: Hola Mundo :D
pub fn say_hello() {
    println!("Hola Mundo :D");
}