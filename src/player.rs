#[deriving(Eq, PartialEq, Show)]
pub struct Player;

impl Player {
    pub fn new() -> Player {
        Player
    }
}

struct PlayerTurn<'a> {
    current_player: uint,
    players: &'a [Player],
}

impl<'a> PlayerTurn<'a> {
    fn new(players: &'a [Player]) -> PlayerTurn<'a> {
        PlayerTurn {
            current_player: 0,
            players: players,
        }
    }

    fn next(&mut self) -> &Player {
        self.current_player = (self.current_player + 1) % self.players.len();
        self.current()
    }

    fn current(&self) -> &Player {
        &self.players[self.current_player]
    }
}

#[cfg(test)]
mod test {
    use super::{Player, PlayerTurn};

    fn make_players(n: uint) -> Vec<Player> {
        Vec::from_fn(n, |_| Player::new())
    }

    #[test]
    fn current_player_is_returned() {
        let players = make_players(2);
        let order = PlayerTurn::new(players.as_slice());
        assert_eq!(players[0], *order.current());
        assert_eq!(players[0], *order.current());
    }

    #[test]
    fn next_player_is_returned() {
        let players = make_players(3);
        let mut order = PlayerTurn::new(players.as_slice());
        assert_eq!(players[1], *order.next());
        assert_eq!(players[1], *order.current());
    }

    #[test]
    fn next_player_wraps_around() {
        let players = make_players(3);
        let mut order = PlayerTurn::new(players.as_slice());
        assert_eq!(players[1], *order.next());
        assert_eq!(players[2], *order.next());
        assert_eq!(players[0], *order.next());
    }
}
