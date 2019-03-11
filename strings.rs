// primitive strings vs
// mutable, heap allocated strings
pub fn run() {
    let mut primitive_str = "hello";
    primitive_str = "bye";
    println!("I said {}", primitive_str);
    let mut growable_str = String::from("Hello");
    growable_str.push_str(" Sir");
    println!("I said {}", growable_str);

    // No need of apache stringUtils libraries
    for word in growable_str.split_whitespace() {
        println!("{}", word);
    }

    // create a string with initial capacity

    let mut init_size = String::with_capacity(10);
    init_size.push_str("January is first");
    println!("{}",init_size);

}
