struct Wrapper<'a, T> {
    value: &'a T,
}

fn main() {
    let data = String::from("Cached response");
    let wrapper = Wrapper { value: &data };

    println!("Wrapped value: {}", wrapper.value);
}
