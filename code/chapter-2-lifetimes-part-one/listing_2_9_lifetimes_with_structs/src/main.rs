struct Person {
    name: &str,
    age: u8,
}

fn main() {
    let frank = Person {
        name: "Frank",
        age: 34,
    };

    println!("User name: {:?}", frank.name);
    println!("User age: {:?}", frank.age);
}
