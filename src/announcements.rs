use bonuses::{BonusType, valid_bonuses};
use cards::CardSuit;
use player::{PlayerTurn, Player, PlayerId};

use std::collections::HashSet;

// Next player to announce the bonuses or last if the player is the last announcer.
#[deriving(Show, Eq, PartialEq)]
pub enum Success {
    Last,
    Next(PlayerId),
}

// An error indicating a problem with the announced bonuses.
#[deriving(Show, Eq, PartialEq)]
pub enum AnnounceError {
    Done,
    NotPlayersTurn,
    InvalidBonus,
}

const NUM_PLAYERS: uint = 4;

// Handling of player bonus announcements in the right order.
struct Announcements {
    turn: PlayerTurn,
    done: bool,
    king: Option<CardSuit>,
}

impl Announcements {
    // Constructs a new announcement handler with the declaring player
    // without a called king.
    // Use for contracts that do not include calling a king (solo contracts).
    pub fn new(declarer: &Player) -> Announcements {
        Announcements {
            turn: PlayerTurn::start_with(NUM_PLAYERS, declarer.id()),
            done: false,
            king: None,
        }
    }

    // Constructs a new announcement handler with the declaring player
    // with a called king.
    pub fn with_king(declarer: &Player, king: CardSuit) -> Announcements {
        let mut ann = Announcements::new(declarer);
        ann.king = Some(king);
        ann
    }

    // The player that is currently announcing.
    pub fn current_player(&self) -> PlayerId {
        *self.turn.current()
    }

    // Announce bonuses for the player.
    pub fn announce(&mut self, player: &Player, bonuses: &HashSet<BonusType>) -> Result<Success, AnnounceError> {
        if self.is_done() {
            Err(Done)
        } else if *self.turn.current() != player.id() {
            Err(NotPlayersTurn)
        } else if !check_bonuses_valid(player, bonuses, self.king) {
            Err(InvalidBonus)
        } else {
            Ok(self.next_player())
        }
    }

    // Pass announcing bonuses for the player.
    pub fn pass(&mut self, player: &Player) -> Result<Success, AnnounceError> {
        if self.is_done() {
            Err(Done)
        } else if *self.turn.current() != player.id() {
            Err(NotPlayersTurn)
        } else {
            Ok(self.next_player())
        }
    }

    // Move the announcing to the next player after successful player announcement.
    fn next_player(&mut self) -> Success {
        if self.turn.current_players() > 1 {
            self.turn.remove();
            Next(*self.turn.current())
        } else {
            self.done = true;
            Last
        }
    }

    // Returns true if the announcements are finished.
    pub fn is_done(&self) -> bool {
        self.done
    }
}

// Check if the announced bonuses for the player are valid.
fn check_bonuses_valid(player: &Player, bonuses: &HashSet<BonusType>, king: Option<CardSuit>) -> bool {
    bonuses.is_subset(&valid_bonuses(player, king))
}

#[cfg(test)]
mod test {
    use super::{Announcements, Next, Last, Done, NotPlayersTurn, InvalidBonus};

    use bonuses::*;
    use cards::*;
    use player::Player;

    fn players() -> Vec<Player> {
        vec![
            Player::new(0, Hand::empty()),
            Player::new(1, Hand::new([CARD_TAROCK_PAGAT])),
            Player::new(2, Hand::new([CARD_CLUBS_KING])),
            Player::new(3, Hand::empty()),
        ]
    }

    #[test]
    fn announcing_starts_with_the_declarer() {
        let players = players();
        let ann = Announcements::new(&players[2]);
        assert_eq!(players[2].id(), ann.current_player());
        let ann2 = Announcements::new(&players[3]);
        assert_eq!(players[3].id(), ann2.current_player());
    }

    #[test]
    fn player_can_pass_the_announcement() {
        let players = players();
        let mut ann = Announcements::new(&players[0]);
        assert_eq!(ann.pass(&players[0]), Ok(Next(1)));
    }

    #[test]
    fn player_can_announce() {
        let players = players();
        let mut ann = Announcements::new(&players[0]);
        assert_eq!(ann.announce(&players[0], &set![Kings]), Ok(Next(1)));
    }

    #[test]
    fn announcements_are_done_when_all_player_either_pass_or_announce() {
        let players = players();
        let mut ann = Announcements::new(&players[0]);
        assert_eq!(ann.announce(&players[0], &set![Kings]), Ok(Next(1)));
        assert_eq!(ann.pass(&players[1]), Ok(Next(2)));
        assert_eq!(ann.pass(&players[2]), Ok(Next(3)));
        assert_eq!(ann.announce(&players[3], &set![Trula]), Ok(Last));
    }

    #[test]
    fn all_players_can_pass() {
        let players = players();
        let mut ann = Announcements::new(&players[0]);
        assert_eq!(ann.pass(&players[0]), Ok(Next(1)));
        assert_eq!(ann.pass(&players[1]), Ok(Next(2)));
        assert_eq!(ann.pass(&players[2]), Ok(Next(3)));
        assert_eq!(ann.pass(&players[3]), Ok(Last));
    }

    #[test]
    fn announcing_or_passing_not_allowed_after_the_announcelements_are_done() {
        let players = players();
        let mut ann = Announcements::new(&players[0]);
        assert_eq!(ann.pass(&players[0]), Ok(Next(1)));
        assert_eq!(ann.pass(&players[1]), Ok(Next(2)));
        assert_eq!(ann.pass(&players[2]), Ok(Next(3)));
        assert_eq!(ann.pass(&players[3]), Ok(Last));
        assert_eq!(ann.pass(&players[3]), Err(Done));
        assert_eq!(ann.announce(&players[3], &set![Kings]), Err(Done));
    }

    #[test]
    fn player_cannot_announce_or_pass_when_its_not_his_turn() {
        let players = players();
        let mut ann = Announcements::new(&players[0]);
        assert_eq!(ann.pass(&players[1]), Err(NotPlayersTurn));
        assert_eq!(ann.announce(&players[2], &set![Kings]), Err(NotPlayersTurn));
    }

    #[test]
    fn king_ultimo_can_only_be_announced_if_the_player_has_the_called_king() {
        let players = players();
        let mut ann = Announcements::with_king(&players[0], Clubs);
        assert_eq!(ann.pass(&players[0]), Ok(Next(1)));
        assert_eq!(ann.announce(&players[1], &set![KingUltimo]), Err(InvalidBonus));
        assert_eq!(ann.pass(&players[1]), Ok(Next(2)));
        assert_eq!(ann.announce(&players[2], &set![KingUltimo]), Ok(Next(3)));
    }

    #[test]
    fn king_ultimo_cannot_be_announced_if_the_contract_does_not_include_king_calling() {
        let players = players();
        let mut ann = Announcements::new(&players[0]);
        assert_eq!(ann.pass(&players[0]), Ok(Next(1)));
        assert_eq!(ann.announce(&players[1], &set![KingUltimo]), Err(InvalidBonus));
        assert_eq!(ann.pass(&players[1]), Ok(Next(2)));
        assert_eq!(ann.announce(&players[2], &set![KingUltimo]), Err(InvalidBonus));
    }

    #[test]
    fn pagat_ultimo_can_only_be_announced_if_the_player_owns_it() {
        let players = players();
        let mut ann = Announcements::new(&players[0]);
        assert_eq!(ann.pass(&players[0]), Ok(Next(1)));
        assert_eq!(ann.announce(&players[1], &set![PagatUltimo]), Ok(Next(2)));
        assert_eq!(ann.announce(&players[2], &set![PagatUltimo]), Err(InvalidBonus));
    }

    #[test]
    fn player_can_announce_multiple_bonuses() {
        let players = players();
        let mut ann = Announcements::new(&players[0]);
        assert_eq!(ann.pass(&players[0]), Ok(Next(1)));
        assert_eq!(ann.announce(&players[1], &set![PagatUltimo, Trula, Kings, Valat]), Ok(Next(2)));
    }
}
