use bonuses::BonusType;
use cards::{Hand, Pile, CardDeal, Talon};
use contracts::Contract;

pub type PlayerId = u64;

// A tarock game player with dealt cards.
#[deriving(Clone)]
pub struct Player {
    id: PlayerId,
    hand: Hand,
    bids: Vec<BonusType>,
    pile: Pile,
    partner: Option<PlayerId>,
}

impl Player {
    // Constructs a new player with an id and dealt hand.
    pub fn new(id: PlayerId, hand: Hand) -> Player {
        Player {
            id: id,
            hand: hand,
            bids: Vec::new(),
            pile: Pile::new(),
            partner: None,
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

    // Returns current bids of the player.
    pub fn bids(&self) -> &[BonusType] {
        self.bids.as_slice()
    }

    // Returns a current pile of cards.
    pub fn pile(&self) -> &Pile {
        &self.pile
    }

    // Returns a mutable reference to a current pile of cards.
    pub fn pile_mut(&mut self) -> &mut Pile {
        &mut self.pile
    }

    // Takes the current pile and returns it. Current pile is replaced by an
    // empty one.
    pub fn take_pile(&mut self) -> Pile {
        use std::mem;
        mem::replace(&mut self.pile, Pile::new())
    }

    // Returns a partner of the player.
    pub fn partner(&self) -> Option<PlayerId> {
        self.partner
    }

    // Set a partner of the player.
    pub fn set_partner(&mut self, id: PlayerId) {
        self.partner = Some(id);
    }
}

// Players of a game.
pub struct Players {
    players: Vec<Player>,
    dealer: uint,
}

impl Players {
    // Constructs new `Players` with the specified number of players.
    // The player with id 0 is the first dealer, players have no cards.
    pub fn new(n: uint) -> Players {
        let players = range(0, n as u64)
            .map(|player_id| Player::new(player_id, Hand::empty()))
            .collect();
        Players {
            players: players,
            dealer: 0,
        }
    }

    // Deals the card packs to the players and returns the talon.
    pub fn deal(&mut self, deal: CardDeal) -> Talon {
        assert!(deal.hands.len() == self.players.len());
        for (player, hand) in self.players.iter_mut().zip(deal.hands.into_iter()) {
            player.hand = hand;
        }
        deal.talon
    }

    // Returns a reference to a player that is current the dealer.
    pub fn dealer(&self) -> &Player {
        &self.players[self.dealer]
    }

    // Constructs a new `ContractPlayers` with specified declarer and contract played.
    pub fn play_contract<'a>(&'a mut self, declarer: PlayerId, contract: Contract) -> ContractPlayers<'a> {
        ContractPlayers {
            declarer: declarer as uint,
            players: self,
            contract: contract,
        }
    }

    // Returns a reference to a player with a given id.
    pub fn player(&self, id: PlayerId) -> &Player {
        &self.players[id as uint]
    }

    // Returns a mutable reference to a player with a given id.
    pub fn player_mut(&mut self, id: PlayerId) -> &mut Player {
        &mut self.players[id as uint]
    }
}

// Players playing a contract.
pub struct ContractPlayers<'a> {
    declarer: uint,
    players: &'a mut Players,
    contract: Contract,
}

impl<'a> ContractPlayers<'a> {
    // Returns a player that is the declarer of currently played contract.
    pub fn declarer(&self) -> &Player {
        self.player(self.declarer as PlayerId)
    }

    // Returns a list of all currently scoring players.
    pub fn scoring_players(&self) -> Vec<&Player> {
        if self.contract.is_klop() {
            self.players.players.iter().collect()
        } else {
            self.scoring_players_normal()
        }
    }

    // Returns a list of all currently scoring player for normal games.
    fn scoring_players_normal(&self) -> Vec<&Player> {
        let declarer_id = self.declarer as PlayerId;
        let mut scoring = vec![self.player(declarer_id)];
        match self.player(declarer_id).partner() {
            Some(partner_id) => { scoring.push(self.player(partner_id)) }
            None => {},
        };
        scoring
    }

    // Returns the currently played contract.
    pub fn contract(&self) -> Contract {
        self.contract
    }

    // Returns a reference to a player with a given id.
    fn player(&self, player_id: PlayerId) -> &Player {
        &self.players.players[player_id as uint]
    }

    // Returns a mutable reference to a player with a given id.
    fn player_mut(&mut self, player_id: PlayerId) -> &mut Player {
        &mut self.players.players[player_id as uint]
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
    use contracts::{SoloWithout, Standard, Two};
    use super::*;

    #[test]
    fn scoring_player_is_returned() {
        let mut players = Players::new(4);
        for declarer in range(0u64, 4) {
            let mut cp = players.play_contract(declarer, SoloWithout);
            let scoring = cp.scoring_players();
            assert_eq!(scoring.len(), 1);
            assert_eq!(scoring[0].id(), declarer);
        }
    }

    #[test]
    fn both_scoring_players_are_returned_with_partner() {
        let mut players = Players::new(4);
        for declarer in range(0u64, 4) {
            for partner in range(0u64, 4) {
                if declarer == partner {
                    continue
                }
                players.player_mut(declarer).set_partner(partner);
                let mut cp = players.play_contract(declarer, Standard(Two));
                let scoring = cp.scoring_players();
                assert_eq!(scoring.len(), 2);
                assert_eq!(scoring[0].id(), declarer);
                assert_eq!(scoring[1].id(), partner);
            }
        }
    }

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
