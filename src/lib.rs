//
// file: lib.rs
// author: Michael Brockus
// gmail: <michaelbrockus@gmail.com>
//

//
// Greet the user
//
pub fn greet() -> &'static str {
    "Greetings Rust programmer, let's get started!"
} // end of function greet

//
// foundation of the program and related
// application logic must be implemented
// in the foundation.
//
pub fn foundation() {
    println!("{message}", message=greet());
} // end of function foundation
