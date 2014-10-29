use std::collections::HashSet;

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

    pub fn pile(&self) -> &Pile {
        &self.pile
    }

    pub fn pile_mut(&mut self) -> &mut Pile {
        &mut self.pile
    }

    pub fn take_pile(&mut self) -> Pile {
        use std::mem;
        mem::replace(&mut self.pile, Pile::new())
    }

    pub fn partner(&self) -> Option<PlayerId> {
        self.partner
    }

    pub fn set_partner(&mut self, id: PlayerId) {
        self.partner = Some(id);
    }
}

pub struct Players {
    players: Vec<Player>,
    dealer: uint,
}

impl Players {
    pub fn new(n: uint) -> Players {
        let players = range(0, n as u64)
            .map(|player_id| Player::new(player_id, Hand::empty()))
            .collect();
        Players {
            players: players,
            dealer: 0,
        }
    }

    pub fn deal(&mut self, deal: CardDeal) -> Talon {
        assert!(deal.hands.len() == self.players.len());
        for (player, hand) in self.players.iter_mut().zip(deal.hands.into_iter()) {
            player.hand = hand;
        }
        deal.talon
    }

    pub fn dealer(&self) -> &Player {
        &self.players[self.dealer]
    }

    pub fn play_contract<'a>(&'a mut self, declarer: PlayerId, contract: Contract) -> ContractPlayers<'a> {
        ContractPlayers {
            declarer: declarer as uint,
            players: self,
        }
    }

    pub fn player(&self, id: PlayerId) -> &Player {
        &self.players[id as uint]
    }

    pub fn player_mut(&mut self, id: PlayerId) -> &mut Player {
        &mut self.players[id as uint]
    }
}

pub struct ContractPlayers<'a> {
    declarer: uint,
    players: &'a mut Players,
}

impl<'a> ContractPlayers<'a> {
    pub fn declarer(&self) -> &Player {
        self.player(self.declarer as PlayerId)
    }

    pub fn scoring_players<'a>(&'a mut self) -> Vec<&'a mut Player> {
        let declarer_id = self.declarer as PlayerId;
        let mut scoring = vec![];
        let (partner_id, found) = match self.player(declarer_id).partner() {
            Some(partner_id) => (partner_id, true),
            None => (0, false),
        };
        // Split the players somewhere between declarer and partner with each
        // being in thei own split part.
        let split_index = ((declarer_id + partner_id) as f64 / 2.0).abs().ceil() as uint;
        // Split the players so we can get two mutable references to the elements.
        let (p1, p2) = self.players.players.as_mut_slice().split_at_mut(split_index);
        if partner_id > declarer_id {
            scoring.push(&mut p1[declarer_id as uint]);
            if found {
                scoring.push(&mut p2[partner_id as uint - split_index]);
            }
        } else {
            scoring.push(&mut p2[declarer_id as uint - split_index]);
            if found {
                scoring.push(&mut p1[partner_id as uint]);
            }
        }
        scoring
    }

    fn player(&self, player_id: PlayerId) -> &Player {
        &self.players.players[player_id as uint]
    }

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
