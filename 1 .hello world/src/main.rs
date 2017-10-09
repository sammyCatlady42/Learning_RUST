//! Hello world

fn main() {
    multiple_hello("World", 2);
}
/// The method mutiple_hello prints "Hello <name>!" n times.
pub fn multiple_hello(name: &str, n: i32) {
    for _ in 0..n {
        println!("Hello, {}!", name);
    }
println!("How r u, {} ?", name);
}
