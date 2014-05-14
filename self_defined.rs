use std::num::Zero;
use std::num::One;

// In this example, we define two traits: `Sum` and `Product`
// that we will then implement on `Vec` in order to easily
// compute the sum and product of the elements inside.

trait Sum<A> {
    fn sum(&self) -> A;
}

trait Product<A> {
    fn product(&self) -> A;
}

// We can only define `Sum` on vectors that have elements
// of type `A`, where `A` has a `Zero` element and `Add`
// defined on it.
impl <A: Zero + Add<A, A>> Sum<A> for std::vec::Vec<A> {
    fn sum(&self) -> A {
        let z: A = Zero::zero();
        self.iter().fold(z, |a, b| a + *b)
    }
}

// We can only implement `Product` on vectors that have elements
// of type `A`, where `A` has a `One` element and `Mul` defined
// on it.
impl <A: One + Mul<A, A>> Product<A> for std::vec::Vec<A> {
    fn product(&self) -> A {
        let z: A = One::one();
        self.iter().fold(z, |a, b| a * *b)
    }
}

fn main() {
    println!("{}", vec![1, 2, 3, 4, 5].sum());
    println!("{}", vec![1, 2, 3, 4, 5].product());
}
