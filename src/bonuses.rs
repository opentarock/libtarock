use cards::{Card, TarockCard, Tarock1, Tarock21, TarockSkis, SuitCard,
    Clubs, Spades, Hearts, Diamonds, King, CardSuit, CARD_TAROCK_PAGAT};
use player::Player;

use std::collections::HashSet;

use contracts::Contract;

pub static BONUS_TYPES: [BonusType, ..5] = [
    Trula,
    Kings,
    KingUltimo,
    PagatUltimo,
    Valat,
];

// Type of point bonus.
#[deriving(Clone, Show, Eq, PartialEq, Hash)]
pub enum BonusType {
    Trula,
    Kings,
    KingUltimo,
    PagatUltimo,
    Valat
}

impl BonusType {
    // Value of bonus
    pub fn value(&self) -> int {
        match *self {
            Trula => 10,
            Kings => 10,
            KingUltimo => 10,
            PagatUltimo => 25,
            Valat => 250,
        }
    }
}

// Bonunes are additional ways to earn points.
#[deriving(Clone, Show)]
pub enum Bonus {
    Unannounced(BonusType),
    Announced(BonusType),
}

impl Bonus {
    // Value of bonus.
    // Announced bonus is worth 2 times more than an announced one.
    pub fn value(&self) -> int {
        match *self {
            Unannounced(bt) => bt.value(),
            Announced(bt) => 2 * bt.value(),
        }
    }

    // Returns true if bonus is announced.
    pub fn is_announced(&self) -> bool {
        match *self {
            Unannounced(_) => false,
            Announced(_) => true,
        }
    }
}

// Checks if cards contain a trula.
pub fn has_trula(cards: &[Card]) -> bool {
    let mut pagat = false;
    let mut mond = false;
    let mut skis = false;
    for card in cards.iter() {
        match *card {
            TarockCard(Tarock1) => pagat = true,
            TarockCard(Tarock21) => mond = true,
            TarockCard(TarockSkis) => skis = true,
            _ => {}
        }
        if pagat && mond && skis {
            return true
        }
    }
    false
}

// Returns true if bonuses are allowed to be announced for the contract.
pub fn bonuses_allowed(contract: &Contract) -> bool {
    contract.is_normal()
}

// Checks if cards contain all four kings.
pub fn has_kings(cards: &[Card]) -> bool {
    let mut clubs = false;
    let mut spades = false;
    let mut hearts = false;
    let mut diamonds = false;
    for card in cards.iter() {
        match *card {
            SuitCard(King, Clubs) => clubs = true,
            SuitCard(King, Spades) => spades = true,
            SuitCard(King, Hearts) => hearts = true,
            SuitCard(King, Diamonds) => diamonds = true,
            _ => {}
        }
        if clubs && spades && hearts && diamonds {
            return true
        }
    }
    false
}

// Returns a set of valid bonuses for the player.
pub fn valid_bonuses(player: &Player, king: Option<CardSuit>) -> HashSet<BonusType> {
    let mut bonuses = HashSet::new();
    // Always valid bonuses.
    bonuses.insert(Trula);
    bonuses.insert(Kings);
    bonuses.insert(Valat);
    if has_king(player, king) {
        bonuses.insert(KingUltimo);
    }
    if has_pagat(player) {
        bonuses.insert(PagatUltimo);
    }
    return bonuses
}

// Returns true if the player owns the king of specified suit.
// If no king is given it always returns false.
fn has_king(player: &Player, king: Option<CardSuit>) -> bool {
   king.map(|suit| player.hand().has_card(SuitCard(King, suit))).unwrap_or(false)
}

// Returns true if the player owns the pagat card.
fn has_pagat(player: &Player) -> bool {
    player.hand().has_card(CARD_TAROCK_PAGAT)
}

#[cfg(test)]
mod test {
    use super::{BONUS_TYPES, Unannounced, Announced, has_trula, has_kings,
        valid_bonuses, Trula, Kings, Valat, KingUltimo, PagatUltimo};

    use cards::*;
    use player::Player;

    #[test]
    fn announced_bonuses_are_worth_two_times_more() {
        for bonus_type in BONUS_TYPES.iter() {
            assert_eq!(2 * Unannounced(*bonus_type).value(), Announced(*bonus_type).value());
        }
    }

    #[test]
    fn succeeds_if_cards_contain_trula() {
        let mut cards = vec!(CARD_CLUBS_KING, CARD_TAROCK_10, CARD_TAROCK_PAGAT,
                             CARD_HEARTS_KING, CARD_DIAMONDS_KING, CARD_CLUBS_EIGHT,
                             CARD_TAROCK_SKIS);
        assert!(!has_trula(cards.as_slice()));
        cards.push(CARD_TAROCK_MOND);
        assert!(has_trula(cards.as_slice()));
    }

    #[test]
    fn succeeds_if_cards_contain_all_four_kings() {
        let mut cards = vec!(CARD_CLUBS_KING, CARD_TAROCK_10, CARD_SPADES_KING,
                             CARD_HEARTS_KING, CARD_DIAMONDS_QUEEN, CARD_CLUBS_EIGHT,
                             CARD_TAROCK_SKIS);
        assert!(!has_kings(cards.as_slice()));
        cards.push(CARD_DIAMONDS_KING);
        assert!(has_kings(cards.as_slice()));
    }

    #[test]
    fn king_ultimo_valid_if_the_player_has_the_called_king() {
        let mut cards = vec!(CARD_CLUBS_KING, CARD_TAROCK_10, CARD_CLUBS_SEVEN,
                             CARD_HEARTS_NINE, CARD_DIAMONDS_NINE, CARD_CLUBS_EIGHT);
        let hand = Hand::new(cards.as_slice());
        let player = Player::new(0, hand);
        assert_eq!(valid_bonuses(&player, Some(Hearts)), set![Trula, Kings, Valat]);
        cards.push(CARD_HEARTS_KING);
        let hand = Hand::new(cards.as_slice());
        let player = Player::new(0, hand);
        assert_eq!(valid_bonuses(&player, Some(Hearts)), set![Trula, Kings, Valat, KingUltimo]);
    }

    #[test]
    fn pagat_ultimo_valid_only_if_the_player_has_the_pagat_card() {
        let mut cards = vec!(CARD_CLUBS_KING, CARD_TAROCK_10, CARD_CLUBS_SEVEN,
                             CARD_HEARTS_NINE, CARD_DIAMONDS_NINE, CARD_CLUBS_EIGHT);
        let hand = Hand::new(cards.as_slice());
        let player = Player::new(0, hand);
        assert_eq!(valid_bonuses(&player, Some(Hearts)), set![Trula, Kings, Valat]);
        cards.push(CARD_TAROCK_PAGAT);
        let hand = Hand::new(cards.as_slice());
        let player = Player::new(0, hand);
        assert_eq!(valid_bonuses(&player, Some(Hearts)), set![Trula, Kings, Valat, PagatUltimo]);
    }
}
