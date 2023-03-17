// Defining a simple feature

trait SimpleFeature {
    // associated type can be anything by definition
    type Output;

    // Function should be implemented by any struct that 
    // impl SimpleFeature
    fn poll(&mut self, wake: fn()) -> Poll<Self::Output>;
}

// defining the Poll enum
// This accepts a template define by type T
// The variants can be either one of type Ready and Pending
enum Poll<T> {
    Ready(T),
    Pending,
}

fn main() {
    println!("Hello, world!");
}
