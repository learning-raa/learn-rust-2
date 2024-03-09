//  //  //  //  //  //  //  //  //  //  //
fn main() {
    let list = [Some(1), Some(10), None, None, Some(100), Some(1000), None];
    println!("\ndbg: {:?}", list);
    let new_1 = list.map(|i: Option<i32>| {
        println!("\nmap list: {:?}", i);
        i.map(|ii| {
            println!("map ii: {:?}", ii);
            "= ".to_owned() + &ii.to_string()
        })
    });
    println!("dbg: {:?}\n", new_1);
    //
    println!("\ndbg: {:?}", list);
    let new_2: [Option<String>; 7] = list.map(|i| {
        println!("\nmap list: {:?}", i);
        i.and_then(|ii| {
            println!("and_then ii: {:?}", ii);
            Some("= ".to_owned() + &ii.to_string())
        })
    });
    println!("dbg: {:?}\n", new_2);
}

//  //  //  //  //  //  //  //  //  //  //
