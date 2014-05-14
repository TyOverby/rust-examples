extern crate sync;
use std::task::spawn;
use sync::Arc;

// Define some mathy functions for later use.
fn sum(vec: &Vec<int>) -> int {
    vec.iter().fold(0, |a, &b| a + b)
}
fn product(vec: &Vec<int>) -> int {
    vec.iter().fold(1, |a, &b| a * b)
}


fn main() {
    // Create an immutable vector of integers.
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    // Wrap it in an 'Arc' for sharing it across threads.
    let num_sync = Arc::new(numbers);

    // Make a channel for our first thread.
    let (input, out) = channel();
    // Send a reference to our thread over.
    input.send(num_sync.clone());
    // Spawn a procedure that prints out the sum of our vectors.
    spawn(proc() {
        let xs = &*out.recv();
        println!("{}", sum(xs));
    });

    // Another channel pair.
    let (input, out) = channel();
    // Send a reference to the same vector
    input.send(num_sync.clone());
    spawn(proc() {
        let xs = &*out.recv();
        // This time print the product.
        println!("{}", product(xs));
    });
}
