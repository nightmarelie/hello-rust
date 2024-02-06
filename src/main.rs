fn main() {
    enum Direction {
        Up,
        Down,
        Left,
        Right,
    }

    // we can use the enum to create a variable
    let player_direction: Direction = Direction::Up;

    // we can also use the enum to create a function
    fn move_player(d: Direction) {
        // match is used to compare the value of d
        match d {
            Direction::Up => println!("Moving up"),
            Direction::Down => println!("Moving down"),
            Direction::Left => println!("Moving left"),
            Direction::Right => println!("Moving right"),
        }
    }

    // call the function with the player_direction variable
    move_player(player_direction);
}
