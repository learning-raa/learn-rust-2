fn main() {
    let my_collection = MyCollection{s:"324",i:0};
    println!("dbg: {:?}", my_collection);
    println!("attempt 1");
    for item in my_collection {
        println!(" item -> {}", item);
    }
}

#[derive(Debug)]
struct MyCollection {
    s: &'static str,
    i: usize,
}

impl Iterator for MyCollection {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let result: Option<char> = self.s.chars().nth(self.i);
        self.i += 1;
        result
    }
}
