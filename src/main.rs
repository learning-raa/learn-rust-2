fn main() {
    let mut arr = [
        EnUm::Text("one".into()),
        EnUm::Text("one2".into()),
        EnUm::Point2 { x: 3, y: 4 },
        EnUm::Text("two".into()),
    ];
    println!("\narr: {:?}\n", arr);
    for item in &mut arr {
        println!("");
        match item {
            EnUm::Point2 { x: ref mut vv, .. } => {
                println!(" BEFORE -> {:?}", vv);
                *vv = 111;
                println!(" AFTER -> {:?}", vv);
            }
            ref val => {
                println!(" a -> {:?}", val);
            }
        }
        println!(" item2 -> {:?}", item);
    }
    println!("\narr: {:?}\n", arr);
}

#[derive(Debug)]
enum EnUm {
    Text(String),
    Point2 { x: i32, y: i32 },
}
