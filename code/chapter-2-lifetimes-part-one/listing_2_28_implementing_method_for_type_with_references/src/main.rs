struct LogEntry<'a, T: 'a> {
    label: &'a str,
    data: T,
}

impl<'a, T: 'a> LogEntry<'a, T> {
    fn describe(&self) {
        println!("{} => entry recorded", self.label);
    }
}

fn main() {
    let label = "event";
    let description = String::from("System rebooted");

    let entry = LogEntry {
        label,
        data: description,
    };

    entry.describe();
}
