use cards::{Card, TarockCard, Tarock1, Tarock21, TarockSkis, SuitCard,
    Clubs, Spades, Hearts, Diamonds, King};

use contracts::Contract;

pub static BONUS_TYPES: [BonusType, ..5] = [
    Trula,
    Kings,
    KingUltimo,
    PagatUltimo,
    Valat,
];

// Type of point bonus.
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

#[cfg(test)]
mod test {
    use super::{BONUS_TYPES, Unannounced, Announced, has_trula, has_kings};

    use cards::*;

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
}
