pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

pub fn example() {
    let mut ac = AveragedCollection {
        list: vec![],
        average: 0.0,
    };

    ac.add(5);
    ac.add(10);
    ac.add(15);

    println!("Average: {}", ac.average());
}
