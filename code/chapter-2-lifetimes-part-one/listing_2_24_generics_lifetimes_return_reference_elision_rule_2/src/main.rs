fn echo<T>(input: &T) -> &T {
    input
}

fn main() {
    let warning = "Overheating!";
    let repeated = echo(&warning);
    println!("Echo: {}", repeated);
}
