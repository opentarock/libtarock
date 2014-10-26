use contracts::{Contract, STANDARD_THREE};
use player::{PlayerId, PlayerTurn};

#[deriving(Eq, PartialEq, Show)]
pub enum Success {
    Next(PlayerId),
    Last,
}

#[deriving(Eq, PartialEq, Show)]
pub enum BidError {
    NotPlayersTurn,
    ContractTooLow,
    InvalidContract,
    MustBid,
    Done,
}

// The `Bidding` trait is used to specify the process of bidding for different
// game variants.
pub trait Bidding {
    // Return the player's id that is currently bidding.
    fn current_player(&self) -> &PlayerId;

    // Bid a contract for a player.
    fn bid(&mut self, player: &PlayerId, contract: Contract) -> Result<Success, BidError>;

    // Pass the bid for a player.
    fn pass(&mut self, player: &PlayerId) -> Result<Success, BidError>;

    // Return true if the bidding process is finished.
    fn is_done(&self) -> bool;

    // Returns the winning bid after the bidding is done, returns `None` otherwise.
    fn winner(&self) -> Option<Bid>;
}

// A bid of a player.
#[deriving(Eq, PartialEq, Show)]
pub struct Bid {
    player: PlayerId,
    player_priority: uint,
    contract: Contract,
}

impl Bid {
    // Constructs a new bid for a player with priority and the bid contract.
    fn new(player: PlayerId, priority: uint, contract: Contract) -> Bid {
        Bid {
            player: player,
            player_priority: priority,
            contract: contract,
        }
    }

    // Return the contract that was bid by the player.
    pub fn contract(&self) -> Contract {
        self.contract
    }

    // Returns the player id of the player that made the bid.
    pub fn player(&self) -> PlayerId {
        self.player
    }
}

// A 4-player bidding helper.
struct Bidder {
    forehand: PlayerId,
    done: bool,
    highest: Bid,
    turn: PlayerTurn,
}

// Default contract for the forehand player.
const DEFAULT_CONTRACT: Contract = STANDARD_THREE;

// The number of players that Bidder is implemented for.
const NUM_PLAYERS: uint = 4;

impl Bidder {
    // Create a new 4-player implementation of Bidding.
    pub fn new(dealer: PlayerId) -> Bidder {
        let mut turn = PlayerTurn::start_with(NUM_PLAYERS, dealer);
        // Skip the dealer as he is the last one to bid.
        turn.next();
        let highest_bid = Bid::new(*turn.current(), player_priority(&turn, turn.current()), DEFAULT_CONTRACT);
        let forehand = *turn.current();
        // Skip the first player because he has a default bid assigned and bids
        // after everybody else.
        turn.next();
        Bidder {
            forehand: forehand,
            done: false,
            highest: highest_bid,
            turn: turn,
        }
    }

    // Returns the current highest bid.
    pub fn current_bid(&self) -> &Bid {
        &self.highest
    }

    // Returns true if forehand player is bidding and the only bid is the default.
    fn has_no_bets(&self, player: &PlayerId) -> bool {
        &self.forehand == player && self.highest.contract() == DEFAULT_CONTRACT
    }

    fn next_player(&mut self, f: |&mut PlayerTurn| -> PlayerId) -> Success {
        if self.turn.current_players() == 1 {
            // Now that the last remaining player bidding has bid we are done.
            self.done = true;
            Last
        } else {
            Next(f(&mut self.turn))
        }
    }
}

impl Bidding for Bidder {
    fn current_player(&self) -> &PlayerId {
        self.turn.current()
    }

    fn bid(&mut self, player: &PlayerId, contract: Contract) -> Result<Success, BidError> {
        let bid = Bid::new(*self.turn.current(), player_priority(&self.turn, player), contract);
        if self.is_done() {
            Err(Done)
        } else if self.turn.current() != player {
            Err(NotPlayersTurn)
        } else if contract.is_klop() && !self.has_no_bets(player) {
            // Klop cannot be played by everyone except the forehand player when
            // no other bids are made.
            Err(InvalidContract)
        } else if !is_bid_valid(&self.highest, &bid){
            Err(ContractTooLow)
        } else {
            self.highest = bid;
            Ok(self.next_player(|turn| *turn.next()))
        }
    }

    fn pass(&mut self, player: &PlayerId) -> Result<Success, BidError> {
        if self.is_done() {
            Err(Done)
        } else if self.turn.current() != player {
            Err(NotPlayersTurn)
        } else if self.has_no_bets(player) || self.turn.current_players() == 1 {
            // Bidding is mandatory if there were no bids made or the last
            // player bidding did not bid yet.
            Err(MustBid)
        } else {
            Ok(self.next_player(|turn| {
                // Player that passes the bid cannot rejoin the bidding again.
                *turn.remove()
            }))
        }
    }

    fn is_done(&self) -> bool {
        self.done
    }

    fn winner(&self) -> Option<Bid> {
        if self.is_done() {
            Some(self.highest)
        } else {
            None
        }
    }
}

fn player_priority(turn: &PlayerTurn, player: &PlayerId) -> uint {
    let pos_diff = *player as uint - *turn.started_with() as uint;
    (pos_diff + turn.num_players() - 1) % turn.num_players()
}

fn is_bid_valid(highest: &Bid, wanted: &Bid) -> bool {
    if wanted.player_priority <= highest.player_priority {
        wanted.contract >= highest.contract
    } else {
        wanted.contract > highest.contract
    }
}

#[cfg(test)]
mod test {
    use super::{Bidder, Bidding, Next, Last, NotPlayersTurn,
        MustBid, Done, InvalidContract, ContractTooLow};

    use super::DEFAULT_CONTRACT;
    use contracts::{KLOP, STANDARD_THREE, STANDARD_TWO, STANDARD_ONE,
        SOLO_THREE, SOLO_TWO, SOLO_ONE};

    #[test]
    fn forehand_player_has_default_bid() {
        let bidder = Bidder::new(0);
        assert_eq!(bidder.current_bid().contract(), DEFAULT_CONTRACT);
        assert_eq!(*bidder.current_player(), 2);
    }

    #[test]
    fn player_can_pass_a_bid() {
        let mut bidder = Bidder::new(0);
        assert_eq!(bidder.current_bid().contract(), DEFAULT_CONTRACT);
        assert!(bidder.pass(&2).is_ok())
        assert_eq!(bidder.current_bid().contract(), DEFAULT_CONTRACT);
        assert!(bidder.pass(&3).is_ok())
        assert_eq!(bidder.current_bid().contract(), DEFAULT_CONTRACT);
    }

    #[test]
    fn player_cant_pass_if_its_not_his_turn() {
        let mut bidder = Bidder::new(0);
        assert_eq!(bidder.pass(&1), Err(NotPlayersTurn));
        assert_eq!(bidder.pass(&3), Err(NotPlayersTurn));
    }

    #[test]
    fn forehand_player_is_not_allowed_to_pass_the_bid_if_no_bids_were_made() {
        let mut bidder = Bidder::new(0);
        assert!(bidder.pass(&2).is_ok())
        assert!(bidder.pass(&3).is_ok())
        assert!(bidder.pass(&0).is_ok())
        assert_eq!(bidder.pass(&1), Err(MustBid));
    }

    #[test]
    fn passing_is_not_allowed_when_bidding_is_finished() {
        let mut bidder = Bidder::new(0);
        assert!(bidder.pass(&2).is_ok())
        assert!(bidder.pass(&3).is_ok())
        assert!(bidder.pass(&0).is_ok())
        assert_eq!(bidder.bid(&1, DEFAULT_CONTRACT), Ok(Last));
        assert_eq!(bidder.pass(&2), Err(Done));
    }

    #[test]
    fn player_can_bid() {
        let mut bidder = Bidder::new(0);
        assert_eq!(bidder.bid(&2, STANDARD_TWO), Ok(Next(3)))
    }

    #[test]
    fn play_is_not_allowed_to_bid_three_of_klop() {
        let mut bidder = Bidder::new(0);
        assert_eq!(bidder.bid(&2, KLOP), Err(InvalidContract))
        assert_eq!(bidder.bid(&2, STANDARD_THREE), Err(ContractTooLow))
    }

    #[test]
    fn forehand_player_can_bid_klop_if_no_other_bids_are_made() {
        let mut bidder = Bidder::new(0);
        assert!(bidder.pass(&2).is_ok())
        assert!(bidder.pass(&3).is_ok())
        assert!(bidder.pass(&0).is_ok())
        assert_eq!(bidder.bid(&1, KLOP), Ok(Last));
    }

    #[test]
    fn player_must_bid_a_higher_bid_than_the_highest() {
        let mut bidder = Bidder::new(0);
        assert_eq!(bidder.bid(&2, STANDARD_TWO), Ok(Next(3)));
        assert_eq!(bidder.bid(&3, STANDARD_TWO), Err(ContractTooLow));
        assert_eq!(bidder.bid(&3, STANDARD_ONE), Ok(Next(0)));
    }

    #[test]
    fn forehand_player_can_bid_contracts_of_equal_or_higher_value() {
        let mut bidder = Bidder::new(0);
        assert_eq!(bidder.bid(&2, STANDARD_TWO), Ok(Next(3)));
        assert_eq!(bidder.pass(&3), Ok(Next(0)));
        assert_eq!(bidder.pass(&0), Ok(Next(1)));
        assert_eq!(bidder.bid(&1, STANDARD_TWO), Ok(Next(2)));
    }

    #[test]
    fn bidding_continues_until_all_players_but_one_pass_the_bid() {
        let mut bidder = Bidder::new(0);
        assert_eq!(bidder.bid(&2, STANDARD_TWO), Ok(Next(3)));
        assert_eq!(bidder.pass(&3), Ok(Next(0)));
        assert_eq!(bidder.bid(&0, STANDARD_ONE), Ok(Next(1)));
        assert_eq!(bidder.bid(&1, SOLO_THREE), Ok(Next(2)));
        assert_eq!(bidder.pass(&2), Ok(Next(0)));
        assert_eq!(bidder.bid(&0, SOLO_TWO), Ok(Next(1)));
        assert_eq!(bidder.pass(&1), Ok(Next(0)));
        assert_eq!(bidder.bid(&0, SOLO_ONE), Ok(Last));
    }

    #[test]
    fn bidding_starts_with_next_player_to_dealer() {
        let mut bidder = Bidder::new(3);
        assert_eq!(bidder.bid(&1, STANDARD_TWO), Ok(Next(2)));
    }

    #[test]
    fn winner_bids_last() {
        let mut bidder = Bidder::new(0);
        assert_eq!(bidder.bid(&2, STANDARD_TWO), Ok(Next(3)));
        assert_eq!(bidder.pass(&3), Ok(Next(0)));
        assert_eq!(bidder.pass(&0), Ok(Next(1)))
        assert_eq!(bidder.bid(&1, STANDARD_TWO), Ok(Next(2)));
        assert_eq!(bidder.bid(&2, STANDARD_ONE), Ok(Next(1)));
        assert_eq!(bidder.pass(&1), Ok(Next(2)));
        assert_eq!(bidder.pass(&2), Err(MustBid));
        assert_eq!(bidder.bid(&2, STANDARD_ONE), Ok(Last));
    }
}
