#[derive(Debug)]
pub struct Player {
    pub name: String,
    score: i32,
}

impl Player {
    pub fn new(name: &str) -> Player {
        return Player {
            name: name.to_string(),
            score: 0,
        }
    }

    pub fn play(&mut self, points: i32) {
        self.score += points;
    }
}

#[derive(Debug)]
pub struct Game {
    pub players: Vec<Player>,
}

pub fn example () -> Game {
    let player1 = Player::new("Oleksii");
    let player2 = Player::new("Anna");

    let mut game = Game {
        players: vec![player1, player2],
    };

    for player in &mut game.players {
        player.play(10);
    }

    return game;
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