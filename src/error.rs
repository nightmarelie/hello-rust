use std::error::Error;
use std::fs::{self, File};
use std::io;
use std::io::ErrorKind;
use std::io::Read;

pub fn examples() {
    // Error handling
    // In this line of code, the program will panic and exit
    // panic!("This is an example of panic"); // panic! is a macro

    enum Result<T, E> {
        Ok(T),
        Err(E),
    }

    let f = File::open("hello.txt").expect("Problem opening the file!"); // unwrap() is a shortcut method for code below

    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!(
                "Problem opening the file: {:?}",
                match error.kind() {
                    ErrorKind::NotFound => match File::create("hello.txt") {
                        Ok(fc) => fc,
                        Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
                    },
                    other_error => {
                        panic!("There was a problem opening the file: {:?}", other_error)
                    }
                }
            )
        }
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

    // error propagation
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        examples();
    }
}

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
