struct Point<T, U> {
    x: T,
    y: U,
}

impl<T> Point<T, T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<i32, i32> {
    fn y(&self) -> &i32 {
        &self.y
    }
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
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
    let b = Point { x: 1, y: 'c' };

    let b = p.mixup(b);

    println!("a.x = {}, a.y = {}", a.x(), a.y);
    println!("n.x = {}, n.y = {}", n.x, n.y);
    println!("b.x = {}, b.y = {}", b.x, b.y);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        run();
    }
}
