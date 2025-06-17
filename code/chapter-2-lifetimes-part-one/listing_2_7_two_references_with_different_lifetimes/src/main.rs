struct Config {
    title: String,
    max_connections: usize,
}

fn get_config_title<'a, 'b>(config: &'a Config, validation_note: &'b str) -> &'a str {
    println!("Validation note: {}", validation_note);
    &config.title
}

fn main() {
    let config = Config {
        title: "Main Database".to_string(),
        max_connections: 100,
    };

    let title_ref;

    {
        let note = String::from("Config passed basic validation");
        title_ref = get_config_title(&config, &note);
    }

    println!("Config title: {}", title_ref);
}
