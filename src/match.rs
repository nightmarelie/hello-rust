enum Language {
    Rust,
    Go,
    Python,
    JavaScript,
}

pub fn example () {
    let lang = Language::Rust;

    match lang {
        Language::Rust => println!("Rust is awesome!"),
        Language::Go => println!("Go is cool!"),
        Language::Python => println!("Python is great!"),
        Language::JavaScript => println!("JavaScript is fun!"),
        // _ => println!("Some other language!"), // This is not needed because we are matching all the variants. The _ pattern will match all values (catchall pattern).
    }
}
