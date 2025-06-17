struct Config {
    title: String,
    max_connections: usize,
}

fn return_note<'a, 'b>(config: &'a Config, validation_note: &'b str) -> &'b str {
    println!("Using config title: {}", config.title);
    validation_note
}

fn main() {
    let config = Config {
        title: "Main Database".to_string(),
        max_connections: 100,
    };

    let note_ref;

    {
        let note = String::from("This will be returned");
        note_ref = return_note(&config, &note); // borrowing from short-lived note
    }

    println!("Returned note: {}", note_ref); // ERROR: `note` was dropped
}
