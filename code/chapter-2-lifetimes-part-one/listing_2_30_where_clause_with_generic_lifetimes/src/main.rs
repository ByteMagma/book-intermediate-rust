struct ConfigEntry<'a, T> {
    key: &'a str,
    value: T,
}

impl<'a, T> ConfigEntry<'a, T>
where
    T: std::fmt::Display + 'a,
{
    fn show(&self) {
        println!("{} => {}", self.key, self.value);
    }
}

fn main() {
    let key = "timeout";
    let value = 30;

    let entry = ConfigEntry {
        key,
        value,
    };

    entry.show();
}
