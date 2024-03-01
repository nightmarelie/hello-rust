struct Point<T, U> {
    x: T,
    y: U,
}

enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

pub fn run() {
    let p = Point { x: 5, y: 10 };
    let a = Point { x: 5.1, y: 10.1 };
    let n = Point { x: 1, y: 10.1 };

    println!("p.x = {}, p.y = {}", p.x, p.y);
    println!("a.x = {}, a.y = {}", a.x, a.y);
    println!("n.x = {}, n.y = {}", n.x, n.y);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        run();
    }
}