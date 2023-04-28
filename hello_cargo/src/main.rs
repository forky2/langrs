use lists::first::List;

fn main() {
    println!("Hello, world!");

    let mut list = List::new();
    let list_ptr = &list;
    println!("{:p}", list_ptr);
    for m in 1..1_000_000 {
        list.push(m);
    }
}
