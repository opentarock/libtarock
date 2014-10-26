use std::mem;

use cards::{Card, CardSuit, Trick};
use contracts::{ContractType, Contract, Standard, standard_winner_strategy,
    standard_move_validator};
use player::{Player, PlayerTurn, PlayerId};

#[deriving(Show, PartialEq)]
pub enum Success {
    Next(PlayerId),
    Last,
}

#[deriving(Show, PartialEq)]
pub enum MoveError {
    NotPlayersTurn,
    InvalidCard,
    Done,
}

pub type PlayResult = Result<Success, MoveError>;

// The `ContractGame` trait is used to represent a contract game of slovenian tarock.
pub trait ContractGame {
    // Play a card for the active player.
    // Can only be called while the game is not finished, after that the response
    // will always be `Done` error.
    fn play_card(&mut self, player: PlayerId, card: Card) -> PlayResult;

    // Returns the current contract that is played.
    fn contract(&self) -> Contract;

    // Returns the trick number.
    // Counting starts at 1. A maximum of 12 tricks can be played in a 4-player game.
    fn trick_number(&self) -> uint;

    // Trtuens true if the last trick was played and the game is finished (no cards left to play).
    fn is_finished(&self) -> bool;
}

const NUM_PLAYERS: uint = 4;

// Implementation of `ContractGame` for standard contracts of `Three`, `Two` and `One`.
pub struct StandardGame<'a> {
    players: &'a mut [Player],
    // The type of standard contract.
    contract_type: ContractType,
    // The suit of called king.
    called_king: CardSuit,
    // Current trick.
    trick: Trick,
    turn: PlayerTurn,
    talon: Vec<Card>,
    trick_number: uint,
    done: bool,
}

impl<'a> StandardGame<'a> {
    // Constructs a new `StandardGame` of specified type and with called king by the
    // bid winner player.
    // The rest of not exchanged talon should be passed as talon.
    pub fn new<'a>(players: &'a mut [Player],
                   ty: ContractType,
                   king: CardSuit,
                   talon: Vec<Card>) -> StandardGame<'a> {

        let turn = PlayerTurn::start_with(NUM_PLAYERS, 1);
        StandardGame {
            players: players,
            contract_type: ty,
            called_king: king,
            trick: Trick::empty(),
            turn: turn,
            talon: talon,
            trick_number: 1,
            done: false,
        }
    }

    // Returns a reference to the current active player.
    fn current_player(&self) -> &Player {
        &self.players[*self.turn.current() as uint]
    }

    // Returns a mutable reference to the current active player.
    fn current_player_mut(&mut self) -> &mut Player {
        &mut self.players[*self.turn.current() as uint]
    }
}

impl<'a> ContractGame for StandardGame<'a> {
    fn play_card(&mut self, player: PlayerId, card: Card) -> PlayResult {
        if self.is_finished() {
            Err(Done)
        } else if player != *self.turn.current() {
            Err(NotPlayersTurn)
        } else if !standard_move_validator(self.current_player().hand(), &self.trick, &card) {
            Err(InvalidCard)
        } else {
            // Remove the played card from the player's hand.
            self.current_player_mut().hand_mut().remove_card(&card);
            // Add the played card to the current trick.
            self.trick.add_card(card);
            if self.trick.count() == NUM_PLAYERS {
                // The trick is finished (all players have played the card).
                {
                    let winner = self.trick.winner(standard_winner_strategy);
                    let player = &mut self.players[to_player_index(&self.turn, winner.card_index)];
                    // Start with a fresh trick.
                    let trick = mem::replace(&mut self.trick, Trick::empty());
                    // Add the won trick to the player's pile of cards.
                    player.pile_mut().add_trick(trick);
                    // Next active player is the winner of this trick.
                    self.turn = PlayerTurn::start_with(NUM_PLAYERS, player.id());
                    self.trick_number += 1;
                }
                // We a re done if all the cards have been played.
                self.done = self.current_player().hand().is_empty();
                if self.is_finished() {
                    Ok(Last)
                } else {
                    Ok(Next(*self.turn.current()))
                }
            } else {
                Ok(Next(*self.turn.next()))
            }
        }
    }

    fn contract(&self) -> Contract {
        Standard(self.contract_type)
    }

    fn trick_number(&self) -> uint {
        self.trick_number
    }

    fn is_finished(&self) -> bool {
        self.done
    }
}

// Convert a winning card index to the player index.
fn to_player_index(turn: &PlayerTurn, card_index: uint) -> uint {
    (*turn.started_with() as uint + card_index) % turn.num_players()
}

#[cfg(test)]
mod test {
    use cards::*;
    use contracts::{Three, Standard};
    use player::Player;

    use super::{StandardGame, ContractGame, NotPlayersTurn, Next, InvalidCard,
        Done, Last};

    fn players() -> Vec<Player> {
        vec![
            Player::new(0, Hand::empty()),
            Player::new(1, Hand::empty()),
            Player::new(2, Hand::empty()),
            Player::new(3, Hand::empty()),
        ]
    }

    #[test]
    fn played_contract_is_returned() {
        let mut players = players();
        let game = StandardGame::new(players.as_mut_slice(), Three, Hearts, vec![]);
        assert_eq!(game.contract(), Standard(Three));
    }

    #[test]
    fn only_the_active_player_can_play_the_card() {
        let mut players = vec![
            Player::new(0, Hand::empty()),
            Player::new(1, Hand::new([CARD_TAROCK_10])),
            Player::new(2, Hand::empty()),
            Player::new(3, Hand::new([CARD_TAROCK_MOND])),
        ];
        let mut game = StandardGame::new(players.as_mut_slice(), Three, Hearts, vec![]);
        assert_eq!(game.play_card(3, CARD_TAROCK_MOND), Err(NotPlayersTurn));
        assert_eq!(game.play_card(1, CARD_TAROCK_10), Ok(Next(2)));
    }

    #[test]
    fn player_cant_play_invalid_card() {
        let mut players = vec![
            Player::new(0, Hand::empty()),
            Player::new(1, Hand::new([CARD_TAROCK_10, CARD_HEARTS_NINE])),
            Player::new(2, Hand::new([CARD_HEARTS_JACK, CARD_CLUBS_EIGHT])),
            Player::new(3, Hand::new([CARD_TAROCK_MOND, CARD_SPADES_JACK])),
        ];
        let mut game = StandardGame::new(players.as_mut_slice(), Three, Hearts, vec![]);
        assert_eq!(game.play_card(1, CARD_HEARTS_NINE), Ok(Next(2)));
        // Playing a card that is not valid for the current trick.
        assert_eq!(game.play_card(2, CARD_CLUBS_EIGHT), Err(InvalidCard));
        assert_eq!(game.play_card(2, CARD_HEARTS_JACK), Ok(Next(3)));
        //// Playing a card that the user does not have in his hand.
        assert_eq!(game.play_card(3, CARD_DIAMONDS_NINE), Err(InvalidCard));
        assert_eq!(game.play_card(3, CARD_TAROCK_MOND), Ok(Next(0)));
    }

    #[test]
    fn played_card_is_removed_from_hand() {
        let mut players = vec![
            Player::new(0, Hand::new([CARD_TAROCK_SKIS, CARD_HEARTS_EIGHT])),
            Player::new(1, Hand::new([CARD_TAROCK_10, CARD_HEARTS_NINE])),
            Player::new(2, Hand::new([CARD_HEARTS_JACK, CARD_CLUBS_EIGHT])),
            Player::new(3, Hand::new([CARD_TAROCK_MOND, CARD_SPADES_JACK])),
        ];
        let mut game = StandardGame::new(players.as_mut_slice(), Three, Hearts, vec![]);
        assert_eq!(game.play_card(1, CARD_HEARTS_NINE), Ok(Next(2)));
        assert_eq!(game.play_card(2, CARD_HEARTS_JACK), Ok(Next(3)));
        assert_eq!(game.play_card(3, CARD_TAROCK_MOND), Ok(Next(0)));
        assert_eq!(game.play_card(0, CARD_HEARTS_EIGHT), Ok(Next(3)));
        // Player 3 is the winner of previous trick.
        assert_eq!(game.play_card(3, CARD_TAROCK_MOND), Err(InvalidCard));
    }

    #[test]
    fn the_player_that_won_the_trick_starts_the_next_trick() {
        let mut players = vec![
            Player::new(0, Hand::new([CARD_TAROCK_SKIS, CARD_HEARTS_EIGHT])),
            Player::new(1, Hand::new([CARD_TAROCK_10, CARD_HEARTS_NINE])),
            Player::new(2, Hand::new([CARD_HEARTS_JACK, CARD_CLUBS_EIGHT])),
            Player::new(3, Hand::new([CARD_TAROCK_MOND, CARD_SPADES_JACK])),
        ];
        let mut game = StandardGame::new(players.as_mut_slice(), Three, Hearts, vec![]);
        assert_eq!(game.play_card(1, CARD_TAROCK_10), Ok(Next(2)));
        assert_eq!(game.play_card(2, CARD_HEARTS_JACK), Ok(Next(3)));
        assert_eq!(game.play_card(3, CARD_TAROCK_MOND), Ok(Next(0)));
        // Player 0 wins the trick and starts the next trick.
        assert_eq!(game.play_card(0, CARD_TAROCK_SKIS), Ok(Next(0)));
    }

    #[test]
    fn game_is_done_when_all_cards_are_played() {
        let mut players = vec![
            Player::new(0, Hand::new([CARD_DIAMONDS_EIGHT])),
            Player::new(1, Hand::new([CARD_HEARTS_NINE])),
            Player::new(2, Hand::new([CARD_DIAMONDS_QUEEN])),
            Player::new(3, Hand::new([CARD_TAROCK_14])),
        ];
        let mut game = StandardGame::new(players.as_mut_slice(), Three, Hearts, vec![]);
        assert_eq!(game.play_card(1, CARD_HEARTS_NINE), Ok(Next(2)));
        assert_eq!(game.play_card(2, CARD_DIAMONDS_QUEEN), Ok(Next(3)));
        assert_eq!(game.play_card(3, CARD_TAROCK_14), Ok(Next(0)));
        assert_eq!(game.play_card(0, CARD_DIAMONDS_EIGHT), Ok(Last));
        assert!(game.is_finished());
        assert_eq!(game.play_card(3, CARD_DIAMONDS_EIGHT), Err(Done));
    }
}
