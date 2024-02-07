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
    fn move_player(d: Direction) -> &'static str {
        // match is used to compare the value of d
        match d {
            Direction::Up => "Moving up",
            Direction::Down => "Moving down",
            Direction::Left => "Moving left",
            Direction::Right => "Moving right",
        }
    }

    // call the function with the player_direction variable
    let move_result = move_player(player_direction);

    // print the result
    println!("{}", move_result);
}
