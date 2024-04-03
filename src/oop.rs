pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
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

pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>, // dyn stands fot dynamic dispatch
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!(
            "Drawing a button with width: {}, height: {}, and label: {}",
            self.width, self.height, self.label
        );
    }
}

pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!(
            "Drawing a select box with width: {}, height: {}, and options: {:?}",
            self.width, self.height, self.options
        );
    }
}

pub fn example1() {
    // Rules of trait objects:
    // 1. We can create a trait object from a type that implements the trait.
    // 2. We can only make trait objects from types that implement the trait.
    // 3. We can't return a trait object from a function unless we use a Box or a similar type to store the object.
    // 4. We can't use trait objects that have a generic type parameter.
    // 5. We can't use trait objects that have a Self type parameter.
    // 6. We can't use trait objects that have an associated type parameter.
    // 7. We can't implement a method on a trait object unless the method is object safe.
    // 8. We can't return a trait object that has a generic type parameter unless we use a Box or a similar type to store the object.
    let screen = Screen {
        components: vec![
            Box::new(Button {
                width: 75,
                height: 10,
                label: String::from("OK"),
            }),
            Box::new(SelectBox {
                width: 50,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
        ],
    };

    screen.run();
}
