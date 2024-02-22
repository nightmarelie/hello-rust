#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub score: i32,
}

impl Player {
    pub fn new(name: &str) -> Player {
        return Player {
            name: name.to_string(),
            score: 0,
        };
    }

    pub fn play(&mut self, points: i32) {
        self.score += points;
    }
}

// Tuple struct
struct Color(i32, i32, i32);

#[derive(Debug)]
pub struct Game {
    pub players: Vec<Player>,
}

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width  * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

}

pub fn example() -> Game {
    let black = Color(0, 0, 0);
    println!("Black: {}, {}, {}", black.0, black.1, black.2);

    // 1. Structure
    // 1.1 Define a structure
    // 1.2 Create an instance of a structure
    // 1.3 Access the properties of a structure
    // 1.4 Define methods for a structure
    let player1 = Player::new("Oleksii");
    let player2 = Player::new("Anna");

    let mut game = Game {
        players: vec![player1, player2],
    };

    for player in &mut game.players {
        player.play(10);
    }

    let mut user1 = User {
        email: String::from("test@test.com"),
        username: String::from("test"),
        active: true,
        sign_in_count: 1,
    };

    let name = user1.username;
    user1.username = String::from("test2");
    println!("The result of structure example is: {}", name);

    let user1 = build_user(&name, "test");
    println!("The result of structure example is: {:?}", user1);

    let user2 = User {
        email: String::from("test@test.com"),
        username: String::from("test"),
        ..user1
    };
    println!("The result of structure example is: {:?}", user2);

    // ----------------
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle::square(10);

    println!("The result of structure example is: {:#?}", rect);
    println!("The result of rect3 example is: {:#?}", rect3);
    println!("The area of the rectangle is {} square pixels.", rect.area());
    println!("Can rect hold rect2? {}", rect.can_hold(&rect2));
    println!("Can rect hold rect? {}", rect2.can_hold(&rect));

    return game;
}

fn build_user(username: &str, email: &str) -> User {
    User {
        email: email.to_owned(),
        username: username.to_owned(),
        active: true,
        sign_in_count: 1,
    }
}

// write test for example
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let game = example();

        assert_eq!(game.players.len(), 2);
    }

    #[test]
    fn test_player() {
        let mut player = Player::new("Oleksii");
        player.play(10);

        assert_eq!(player.score, 10);
    }
}