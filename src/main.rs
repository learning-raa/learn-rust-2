//  //  //  //  //  //  //  //  //  //  //
fn main() {
    let list: [Result<i32, String>; 4] = [Ok(1), Ok(10), Err("irore".to_owned()), Ok(100)];
    println!("\ndbg: {:?}", list);
    let new_1 = list.map(|i| {
        println!("\nmap list: {:?}", i);
        i.map(|ii| {
            println!("map ii: {:?}", ii);
            "= ".to_owned() + &ii.to_string()
        })
    });
    println!("dbg: {:?}\n", new_1);
    //
    /*
    println!("\ndbg: {:?}", list);
    let new_2: [Option<String>; 7] = list.map(|i| {
        println!("\nmap list: {:?}", i);
        i.and_then(|ii| {
            println!("and_then ii: {:?}", ii);
            Some("= ".to_owned() + &ii.to_string())
        })
    });
    println!("dbg: {:?}\n", new_2);
    */
}

//  //  //  //  //  //  //  //  //  //  //
