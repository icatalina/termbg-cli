fn main() {
    let timeout = std::time::Duration::from_millis(20);
    let theme = termbg::theme(timeout);

    match theme {
        Ok(theme) => {
            match theme {
                termbg::Theme::Light => println!("light"),
                termbg::Theme::Dark => println!("dark")
            }
        }
        Err(e) => {
            println!("  Theme: detection failed {:?}", e);
        }
    }
}