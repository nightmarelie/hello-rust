mod closures;

use crate::closures::{Inventory, ShirtColor};

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Blue, ShirtColor::Red],
    };
}
