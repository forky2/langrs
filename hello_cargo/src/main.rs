use lists::second::List;


fn another_function(list: &mut List<i32>) {
    // Modify index 5
    list.get_by_idx_mut(5).map(|value| {
        *value = 19191919;
    });
}

fn main() {
    println!("Hello, world!");

    let mut list = List::new();
    for m in 1..1_000_000 {
        list.push(m);
    }

    // Get index 5
    let val = list.get_by_idx(5);
    if val.is_some() {
        println!("Value: {}", val.unwrap());
    }

    // Modify index 5
    list.get_by_idx_mut(5).map(|value| {
        *value = 42
    });
    
    // Check index 5 changed
    let val = list.get_by_idx(5);
    if val.is_some() {
        println!("Value: {}", val.unwrap());
    }

    // Now I want to change an index, keeping a reference.
    let keep_ref = list.get_by_idx_mut(5);
    if keep_ref.is_some() {
        *keep_ref.unwrap() = 999;
    }

    // Check index 5 changed
    let val = list.get_by_idx(5);
    if val.is_some() {
        println!("Value: {}", val.unwrap());
    }

    another_function(&mut list);
    
    // Check index 5 changed
    let val = list.get_by_idx(5);
    if val.is_some() {
        println!("Value: {}", val.unwrap());
    }

}
