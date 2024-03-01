struct Point<T> {
    x: T,
    y: T,
}

pub fn run() {
    let p = Point { x: 5, y: 10 };
    let a = Point { x: 5.1, y: 10.1 };

    println!("p.x = {}, p.y = {}", p.x, p.y);
    println!("p.x = {}, p.y = {}", a.x, a.y);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        run();
    }
}