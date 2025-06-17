struct Pair<'a> {
    first: &'a str,
    second: &'a str,
}

impl<'a> Pair<'a> {
    fn longer(&self) -> &str {
        if self.first.len() > self.second.len() {
            self.first
        } else {
            self.second
        }
    }
}

fn main() {
    let one = String::from("Short");
    let two = String::from("Much longer string");

    let pair = Pair {
        first: &one,
        second: &two,
    };

    println!("Longer string: {}", pair.longer());
}
