// mod iterators;
//
// fn test() {
//     iterators::example();
// }

// use rand::{Rng, CryptoRng};
// use std::io::*;
//
// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//
//         fn seat_at_table() {}
//     }
//
//     mod serving {
//         fn take_order() {
//             // Relative path
//             super::hosting::add_to_waitlist();
//
//             // Absolute path
//             crate::front_of_house::hosting::add_to_waitlist();
//         }
//
//         fn serve_order() {}
//
//         fn take_payment() {}
//     }
// }
//
// pub fn eat_at_restaurant() {
//     // Absolute path
//     crate::front_of_house::hosting::add_to_waitlist();
//
//     // Relative path
//     front_of_house::hosting::add_to_waitlist();
//
//     // use keyword
//     use self::front_of_house::hosting;
//
//     hosting::add_to_waitlist();
// }
