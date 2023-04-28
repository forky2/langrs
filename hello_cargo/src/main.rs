use lists::first::List;

fn main() {
    println!("Hello, world!");

    let s = 5;
    let s_ptr = &s;
    println!("{:p}", s_ptr);

    // for n in 1i64..100000000000000 {
        let mut list = List::new();
        let list_ptr = &list;
        println!("{:p}", list_ptr);
        for m in 1..1_000_000 {
            list.push(m);
        }
    // }
}
