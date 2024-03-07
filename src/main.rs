use std::sync::{Arc, Mutex};
fn main() {
    let over = OverLoad {};
    println!("\narr: {:?}\n", over);
    over.one();
    OverLoad::one(&over);
    over.two();
    let another = Box::new(OverLoad {});
    println!("\nr: {:?}\n", another);
    another.three();
    //println!("\nr: {:?}\n", another);
    let again = Arc::new(OverLoad {});
    println!("\nr: {:?}\n", again);
    again.four();
    //println!("\nr: {:?}\n", again);
}

#[derive(Debug)]
struct OverLoad {}

impl OverLoad {
    fn one(&self) {
        println!("one: <{:?}>", self);
    }
    fn two(self) {
        println!("two: <{:?}>", self);
    }
    fn three(self: Box<Self>) {
        println!("three: <{:?}>", self);
    }
    fn four(self: Arc<Self>) {
        println!("four: <{:?}>", self);
    }
}
