fn print_label<'a, T: std::fmt::Display>(label: &'a T) {
    println!("Label: {}", label);
}

fn main() {
    let name = String::from("Processor Unit");
    print_label(&name);
}
