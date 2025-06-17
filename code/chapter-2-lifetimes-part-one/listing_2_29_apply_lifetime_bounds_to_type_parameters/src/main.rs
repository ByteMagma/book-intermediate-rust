struct Holder<'a, T: 'a> {
    data: &'a T,
}

impl<'a, T: 'a> Holder<'a, T> {
    fn get(&self) -> &'a T {
        self.data
    }
}

fn main() {
    let value = 42;
    let holder = Holder { data: &value };

    let extracted = holder.get();
    println!("Held value: {}", extracted);
}
