# Undefined Behavior with Raw Pointers and Vectors in Rust

This example showcases a potential pitfall when using raw pointers with vectors in Rust. Directly manipulating a vector's contents via a raw pointer after vector operations can result in unpredictable outcomes and crashes.  The solution highlights safe alternatives.

## Bug
The `bug.rs` file demonstrates the unsafe use of a raw pointer to modify a vector.  This approach lacks the memory safety guarantees offered by Rust's ownership and borrowing system. Changing the vector's contents after obtaining the raw pointer might lead to data corruption or panics.