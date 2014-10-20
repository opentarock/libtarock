use cards::{Hand, Card};

pub type PlayerId = u64;

// A tarock game player with dealt cards.
pub struct Player {
    id: PlayerId,
    hand: Hand,
}

impl Player {
    // Constructs a new player with an id and dealt hand.
    pub fn new(id: PlayerId, hand: Hand) -> Player {
        Player {
            id: id,
            hand: hand,
        }
    }

    // Returns player's id.
    pub fn id(&self) -> PlayerId {
        self.id
    }

    // Returns a reference to the current hand of the player.
    pub fn hand(&self) -> &Hand {
        &self.hand
    }

    // Returns a mutable reference to the current hand of the player.
    pub fn hand_mut(&mut self) -> &mut Hand {
        &mut self.hand
    }

    // Returns a reference to the current cards in hand of the player.
    pub fn cards(&self) -> &[Card] {
        self.hand.cards()
    }
}

pub struct PlayerTurn {
    current_index: uint,
    num_players: uint,
    started_with: PlayerId,
    players: Vec<PlayerId>,
}

impl PlayerTurn {
    pub fn new(num_players: uint) -> PlayerTurn {
        PlayerTurn::start_with(num_players, 0)
    }

    pub fn start_with(num_players: uint, first: PlayerId) -> PlayerTurn {
        PlayerTurn {
            current_index: first as uint,
            num_players: num_players,
            started_with: first,
            players: Vec::from_fn(num_players, |i| i as PlayerId),
        }
    }

    pub fn started_with(&self) -> &PlayerId {
        &self.started_with
    }

    pub fn num_players(&self) -> uint {
        self.num_players as uint
    }

    pub fn current_players(&self) -> uint {
        self.players.len()
    }

    pub fn remove(&mut self) -> &PlayerId {
        if self.current_players() > 1 {
            self.players.remove(self.current_index).unwrap();
            self.current_index %= self.current_players();
        }
        self.current()
    }

    pub fn next(&mut self) -> &PlayerId {
        let next_index = (self.current_index + 1) % self.current_players();
        self.current_index = next_index;
        self.current()
    }

    pub fn current(&self) -> &PlayerId {
        &self.players[self.current_index]
    }
}

#[cfg(test)]
mod test {
    use super::PlayerTurn;

    #[test]
    fn current_player_is_returned() {
        let order = PlayerTurn::new(2);
        assert_eq!(0, *order.current());
        assert_eq!(0, *order.current());
    }

    #[test]
    fn starts_at_chosen_player() {
        let order = PlayerTurn::start_with(3, 2);
        assert_eq!(2, *order.current());
    }

    #[test]
    fn next_player_is_returned() {
        let mut order = PlayerTurn::new(3);
        assert_eq!(1, *order.next());
        assert_eq!(1, *order.current());
    }

    #[test]
    fn next_player_wraps_around() {
        let mut order = PlayerTurn::new(3);
        assert_eq!(1, *order.next());
        assert_eq!(2, *order.next());
        assert_eq!(0, *order.next());
    }

    #[test]
    fn removes_current_player() {
        let mut order = PlayerTurn::new(3);
        order.next();
        assert_eq!(2, *order.remove());
        assert_eq!(0, *order.remove());
        assert_eq!(1, order.current_players())
    }

    #[test]
    fn does_not_remove_the_last_player() {
        let mut order = PlayerTurn::new(2);
        assert_eq!(1, *order.remove());
        assert_eq!(1, *order.remove());
        assert_eq!(1, order.current_players())
    }
}
