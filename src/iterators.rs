#[derive(PartialEq, Debug)]
pub struct Shoe {
    size: u32,
    style: String,
}

struct Counter {
    count: u32,
}

/// Iterator for Counter
///
/// # Examples
///
/// ```
/// let mut counter = Counter::new();
///
/// assert_eq!(counter.next(), Some(1));
/// assert_eq!(counter.next(), Some(2));
/// assert_eq!(counter.next(), Some(3));
/// assert_eq!(counter.next(), Some(4));
/// assert_eq!(counter.next(), Some(5));
/// assert_eq!(counter.next(), None);
/// ```
impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

pub fn example() {
    let vec = vec![1, 2, 3];
    // iterator is lazy, it does not do anything until it is used
    let vec_iter = vec.iter();

    for val in vec_iter {
        println!("Got: {}", val);
    }

    let mut shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 13, style: String::from("sandal") },
        Shoe { size: 10, style: String::from("boot") },
    ];

    shoes.sort_by(|a, b| a.size.cmp(&b.size));
    println!("{:?}", shoes);

    let in_my_size = filters_shoes(shoes, 10);
    println!("{:?}", in_my_size);
}


fn filters_shoes(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

// all iterators implement the Iterator trait
// pub trait Iterator {
//     type Item;
//
//     fn next(&mut self) -> Option<Self::Item>; // associated type
//
//     // default implementation
// }

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_example() {
    //     example();
    // }

    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];

        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);


        let mut v1 = vec![1, 2, 3];

        let mut v1_iter = v1.iter_mut();

        assert_eq!(v1_iter.next(), Some(&mut 1));
        assert_eq!(v1_iter.next(), Some(&mut 2));
        assert_eq!(v1_iter.next(), Some(&mut 3));
        assert_eq!(v1_iter.next(), None);


        let v1 = vec![1, 2, 3];

        let mut v1_iter = v1.into_iter();

        assert_eq!(v1_iter.next(), Some(1));
        assert_eq!(v1_iter.next(), Some(2));
        assert_eq!(v1_iter.next(), Some(3));
        assert_eq!(v1_iter.next(), None);
    }

    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        // sum is a consuming adaptor
        let total: i32 = v1_iter.sum();

        assert_eq!(total, 6);
    }

    #[test]
    fn iterator_map() {
        let v1 = vec![1, 2, 3];

        // map is a lazy adaptor. Collect is a consuming adaptor
        let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

        assert_eq!(v2, vec![2, 3, 4]);
    }

    #[test]
    fn iterator_filter() {
        let shoes = vec![
            Shoe { size: 10, style: String::from("sneaker") },
            Shoe { size: 13, style: String::from("sandal") },
            Shoe { size: 10, style: String::from("boot") },
        ];

        let in_my_size = super::filters_shoes(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe { size: 10, style: String::from("sneaker") },
                Shoe { size: 10, style: String::from("boot") },
            ]
        );
    }

    #[test]
    fn iterator_counter() {
        let mut counter = Counter::new();

        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);
    }

    #[test]
    fn using_other_iterator_trait_methods() {
        let sum: u32 = Counter::new().zip(Counter::new().skip(1))
            .map(|(a, b)| a * b)
            .filter(|x| x % 3 == 0)
            .sum();

        assert_eq!(18, sum);
    }
}

