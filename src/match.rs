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
    }
}
