// Import the 'Ord'ered trait.  Any data type that implements Ord
// will have to define the `lt` (less than) method, which is used
// for the `<` operator.
use std::cmp::Ord;

// a function from (T, T) to T where T is 'Ord'ered.
fn min<T: Ord>(a: T, b: T) -> T {
    if a < b { a } else { b }
}

fn main(){
    // Compute the minimum of several data types that already
    // implement Ord.
    println!("{}", min(4, 10));
    println!("{}", min("abc", "def"));
}
