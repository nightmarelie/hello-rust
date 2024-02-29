use std::fs::File;

pub fn examples() {
    // Error handling
    // In this line of code, the program will panic and exit
    panic!("This is an example of panic"); // panic! is a macro

    enum Result<T, E> {
        Ok(T),
        Err(E),
    }

    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem opening the file: {:?}", error)
        },
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        examples();
    }
}