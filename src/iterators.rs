pub fn example() {
    let vec = vec![1, 2, 3];
    // iterator is lazy, it does not do anything until it is used
    let vec_iter = vec.iter();

    for val in vec_iter {
        println!("Got: {}", val);
    }
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
    // use super::*;

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
    }
}

