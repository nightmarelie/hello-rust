use std::fs::File;
use std::io::ErrorKind;

pub fn examples() {
    // Error handling
    // In this line of code, the program will panic and exit
    // panic!("This is an example of panic"); // panic! is a macro

    enum Result<T, E> {
        Ok(T),
        Err(E),
    }

    let f = File::open("hello.txt").unwrap(); // unwrap() is a shortcut method for code below

    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem opening the file: {:?}", match error.kind() {
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
                },
                other_error => {
                    panic!("There was a problem opening the file: {:?}", other_error)
                }
            })
        },
    };

    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Tried to create file but there was a problem: {:?}", error);
            })
        } else {
            panic!("There was a problem opening the file: {:?}", error);
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        examples();
    }
}