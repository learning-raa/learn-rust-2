macro_rules! raa_1 {
    () => {
        println!("raa1");
    };
}

macro_rules! raa_2 {
    ($txt:ident) => {
        println!("raa2: ({})", stringify!($txt));
        let $txt = "something";
        println!("raa2.2: ({})", $txt);
    };
    [$txt:ident] => {
        println!("raa3: [{}]", stringify!($txt));
        let $txt = "something";
        println!("raa3.3: [{}]", $txt);
    };
}

//  //  //  //  //  //  //  //  //  //  //
fn main() {
    raa_1!();
    raa_2!(rr);
    raa_2! {gg};
}

//  //  //  //  //  //  //  //  //  //  //
#[derive(Debug)]
struct OverLoad {}

impl OverLoad {
    fn one(&self) {
        println!("one: <{:?}>", self);
    }
}
