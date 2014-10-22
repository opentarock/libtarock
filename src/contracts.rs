use std::collections::HashSet;

use cards::{CardSuit, Trick, Hand, Card, TarockCard,
    Tarock1, Tarock21, TarockSkis};

#[deriving(Eq, PartialEq, Show)]
pub enum ContractType {
    Three,
    Two,
    One,
}

pub mod beggar {
    #[deriving(Eq, PartialEq, Show)]
    pub enum Type {
        Normal,
        Open,
    }
}
pub mod valat {
    #[deriving(Eq, PartialEq, Show)]
    pub enum Type {
        Normal,
        Color,
    }
}

pub const KLOP: Contract = Klop;
pub const STANDARD_THREE: Contract = Standard(Three);
pub const STANDARD_TWO: Contract = Standard(Two);
pub const STANDARD_ONE: Contract = Standard(One);
pub const SOLO_THREE: Contract = Solo(Three);
pub const SOLO_TWO: Contract = Solo(Two);
pub const SOLO_ONE: Contract = Solo(One);
pub const BEGGAR_NORMAL: Contract = Beggar(beggar::Normal);
pub const SOLO_WITHOUT: Contract = SoloWithout;
pub const BEGGAR_OPEN: Contract = Beggar(beggar::Open);
pub const VALAT_COLOR: Contract = Valat(valat::Color);
pub const VALAT_NORMAL: Contract = Valat(valat::Normal);

#[deriving(Eq, PartialEq, Show)]
pub enum Contract {
    Klop,
    Standard(ContractType),
    Solo(ContractType),
    Beggar(beggar::Type),
    SoloWithout,
    Valat(valat::Type),
}

impl Contract {
    // Returns true if the contract is klop.
    pub fn is_klop(&self) -> bool {
        match *self {
            Klop => true,
            _ => false,
        }
    }

    // Value of contract. If the contract is lost the value of contract is
    // negative (-value).
    pub fn value(&self) -> int {
        match *self {
            Klop => 70,
            Standard(Three) => 10,
            Standard(Two) => 20,
            Standard(One) => 30,
            Solo(Three) => 40,
            Solo(Two) => 50,
            Solo(One) => 60,
            Beggar(beggar::Normal) => 70,
            SoloWithout => 80,
            Beggar(beggar::Open) => 90,
            Valat(valat::Color) => 125,
            Valat(valat::Normal) => 250,
        }
    }

    // Returns true if the contract is one of the "normal" ones.
    // Normal contracts are Three, Two, One, Solo Three, Solo Two and Solo One.
    pub fn is_normal(&self) -> bool {
        match *self {
            Standard(Three) | Solo(Three) | SoloWithout => true,
            _ => false,
        }
    }
}

impl PartialOrd for Contract {
    fn partial_cmp(&self, other: &Contract) -> Option<Ordering> {
        return self.value().partial_cmp(&other.value())
    }
}

pub fn color_valat_winner_strategy(cards: &[Card]) -> uint {
    find_winner(cards, |card, suit| card.suit() == suit)
}

pub fn standard_winner_strategy(cards: &[Card]) -> uint {
    find_winner(cards, |card, suit| card.suit() == suit || card.is_tarock())
}

fn find_winner(cards: &[Card], cond: |&Card, Option<CardSuit>| -> bool) ->uint {
    let played_suit = cards[0].suit();
    let (winner_index, card) = cards.iter()
        .enumerate()
        .filter(|&(_, card)| cond(card, played_suit))
        .max_by(|&(_, card)| card)
        .unwrap();

    if card.is_tarock() && contains_trula(cards) {
        let (winner_index, _) = cards.iter()
            .enumerate()
            .find(|&(_, card)| card.is_pagat()).unwrap();
        return winner_index
    }

    winner_index
}

fn contains_trula(cards: &[Card]) -> bool {
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
            break
        }
    }
    pagat && mond && skis
}

pub trait MoveValidator {
    fn is_valid(&self, hand: &Hand, trick: &Trick, card: &Card) -> bool;
}

pub fn standard_move_validator(hand: &Hand, trick: &Trick, card: &Card) -> bool {
    let suit = trick.first().and_then(|c| c.suit());
    if trick.is_empty() || suit == card.suit() {
        true
    } else {
        match card.suit() {
            Some(_) => !hand.has_tarock(),
            None => suit.map(|suit| !hand.has_suit(suit)).unwrap_or(true),
        }
    }
}

// TODO: refactor
pub fn negative_contract_move_validator(hand: &Hand, trick: &Trick, card: &Card) -> bool {
    if trick.is_empty() {
        true
    } else {
        let suit = trick.first().and_then(|c| c.suit());
        if suit == card.suit() {
            let max = trick.cards().iter()
                .filter(|card| card.suit() == suit || card.is_tarock())
                .max_by(|card| card)
                .unwrap();
            if card.is_pagat() {
                return contains_mond_and_skis(trick.cards().iter()) || has_only_pagat(hand, card)
            } else if card.is_tarock() &&
                contains_mond_and_skis(trick.cards().iter()) &&
                hand.cards().filter(|card| card.is_pagat()).count() == 1 {

                return false
            }
            card > max || !hand.cards()
                .filter(|card| card.suit() == max.suit())
                .any(|card| card > max)
        } else {
            match card.suit() {
                Some(_) => !hand.has_tarock(),
                _ if card.is_pagat() => contains_mond_and_skis(hand.cards()) || has_only_pagat(hand, card),
                _ => suit.map(|suit| !hand.has_suit(suit)).unwrap_or(true) && !contains_mond_and_skis(trick.cards().iter()),
            }
        }
    }
}

fn has_only_pagat(hand: &Hand, card: &Card) -> bool {
    card.is_pagat() && hand.cards().
        filter(|card| card.is_tarock()).
        count() == 1
}

fn contains_mond_and_skis<'a, C: Iterator<&'a Card>>(mut cards: C) -> bool {
    let mut mond = false;
    let mut skis = false;
    for card in cards {
        match *card {
            TarockCard(Tarock21) => mond = true,
            TarockCard(TarockSkis) => skis = true,
            _ => {}
        }
    }
    mond && skis
}

impl MoveValidator for fn(hand: &Hand, trick: &Trick, card: &Card) -> bool {
    fn is_valid(&self, hand: &Hand, trick: &Trick, card: &Card) -> bool {
        (*self)(hand, trick, card)
    }
}


pub fn valid_moves<V: MoveValidator>(validator: V, hand: &Hand, trick: &Trick) -> HashSet<Card> {
    hand.cards().filter(|card| validator.is_valid(hand, trick, *card)).map(|c| *c).collect()
}

#[cfg(test)]
mod test {
    use cards::*;

    use super::{standard_winner_strategy, color_valat_winner_strategy};
    use super::{valid_moves, negative_contract_move_validator, standard_move_validator};

    static HIGH_HEARTS_NO_TAROCKS: &'static [Card] = [
        CARD_HEARTS_JACK,
        CARD_HEARTS_QUEEN,
        CARD_SPADES_KING,
        CARD_HEARTS_SEVEN,
    ];

    static SPADES: &'static [Card] = [
        CARD_SPADES_KING,
        CARD_SPADES_JACK,
        CARD_SPADES_QUEEN,
        CARD_SPADES_SEVEN,
    ];

    static SPADES2: &'static [Card] = [
        CARD_SPADES_JACK,
        CARD_SPADES_SEVEN,
        CARD_SPADES_KING,
        CARD_SPADES_QUEEN,
    ];

    static SUITS_WITH_TAROCK: &'static [Card] = [
        CARD_HEARTS_JACK,
        CARD_SPADES_KING,
        CARD_HEARTS_SEVEN,
        CARD_TAROCK_PAGAT,
    ];

    static JUST_TAROCKS: &'static [Card] = [
        CARD_TAROCK_9,
        CARD_TAROCK_12,
        CARD_TAROCK_20,
        CARD_TAROCK_6,
    ];

    static TAROCKS_TRULA: &'static [Card] = [
        CARD_TAROCK_SKIS,
        CARD_TAROCK_MOND,
        CARD_TAROCK_6,
        CARD_TAROCK_PAGAT,
    ];

    #[test]
    fn standard_highest_card_of_played_suit_wins() {
        assert_eq!(standard_winner_strategy(SPADES), 0)
    }

    #[test]
    fn standard_higher_card_of_different_suit_has_no_effect() {
        assert_eq!(standard_winner_strategy(HIGH_HEARTS_NO_TAROCKS), 1)
    }


    #[test]
    fn standard_tarock_always_wins() {
        assert_eq!(standard_winner_strategy(SUITS_WITH_TAROCK), 3)
    }

    #[test]
    fn standard_highest_tarock_always() {
        assert_eq!(standard_winner_strategy(JUST_TAROCKS), 2)
    }

    #[test]
    fn standard_pagat_wins_if_trula_is_played() {
        assert_eq!(standard_winner_strategy(TAROCKS_TRULA), 3)
    }

    #[test]
    fn color_valat_played_suit_wins() {
        assert_eq!(color_valat_winner_strategy(SUITS_WITH_TAROCK), 0)
        assert_eq!(color_valat_winner_strategy(HIGH_HEARTS_NO_TAROCKS), 1)
    }

    #[test]
    fn color_valat_highest_card_of_suit_wins() {
        assert_eq!(color_valat_winner_strategy(SPADES), 0)
        assert_eq!(color_valat_winner_strategy(SPADES2), 2)
    }

    #[test]
    fn color_valat_highest_tarock_wins() {
        assert_eq!(color_valat_winner_strategy(JUST_TAROCKS), 2)
    }

    #[test]
    fn color_valat_pagat_wins_if_trula_is_played() {
        assert_eq!(color_valat_winner_strategy(TAROCKS_TRULA), 3)
    }

    fn make_trick(cards: &[Card]) -> Trick {
        let mut trick = Trick::empty();
        for card in cards.iter() {
            trick.add_card(*card);
        }
        trick
    }

    #[test]
    fn move_validator_all_moves_are_valid_on_first_play_in_trick() {
        let cards = set![CARD_TAROCK_2, CARD_SPADES_EIGHT, CARD_DIAMONDS_JACK];
        assert_eq!(valid_moves(standard_move_validator,
                               &Hand::from_iter(cards.iter()), &Trick::empty()), cards);
        assert_eq!(valid_moves(negative_contract_move_validator,
                               &Hand::from_iter(cards.iter()), &Trick::empty()), cards);
    }

    #[test]
    fn move_validator_card_of_same_suit_must_be_played() {
        let cards = set![CARD_TAROCK_2, CARD_SPADES_EIGHT, CARD_DIAMONDS_JACK];
        assert_eq!(valid_moves(standard_move_validator,
                               &Hand::from_iter(cards.iter()),
                               &make_trick([CARD_SPADES_KING])),
                               set![CARD_SPADES_EIGHT]);
        assert_eq!(valid_moves(negative_contract_move_validator,
                               &Hand::from_iter(cards.iter()),
                               &make_trick([CARD_SPADES_KING])),
                               set![CARD_SPADES_EIGHT]);
    }

    #[test]
    fn move_validator_tarock_must_be_played_if_no_cards_of_suit_in_hand() {
        let cards = set![CARD_TAROCK_2, CARD_HEARTS_KING, CARD_TAROCK_SKIS, CARD_SPADES_JACK];
        assert_eq!(valid_moves(standard_move_validator,
                               &Hand::from_iter(cards.iter()),
                               &make_trick([CARD_DIAMONDS_KING])),
                               set![CARD_TAROCK_2, CARD_TAROCK_SKIS]);
        assert_eq!(valid_moves(negative_contract_move_validator,
                               &Hand::from_iter(cards.iter()),
                               &make_trick([CARD_DIAMONDS_KING])),
                               set![CARD_TAROCK_2, CARD_TAROCK_SKIS]);
    }

    #[test]
    fn move_validator_other_suits_can_be_played_only_when_required_suit_or_tarocks_missing() {
        let cards = set![CARD_HEARTS_KING, CARD_DIAMONDS_JACK];
        assert_eq!(valid_moves(standard_move_validator,
                               &Hand::from_iter(cards.iter()),
                               &make_trick([CARD_CLUBS_NINE, CARD_CLUBS_KING])),
                               cards);
        assert_eq!(valid_moves(negative_contract_move_validator,
                               &Hand::from_iter(cards.iter()),
                               &make_trick([CARD_CLUBS_NINE, CARD_CLUBS_KING])),
                               cards);
    }

    #[test]
    fn negative_contract_validator_higher_card_of_suit_must_be_played() {
        let cards = set![CARD_TAROCK_13, CARD_SPADES_EIGHT, CARD_SPADES_QUEEN];
        assert_eq!(valid_moves(negative_contract_move_validator,
                               &Hand::from_iter(cards.iter()),
                               &make_trick([CARD_SPADES_KNIGHT, CARD_SPADES_SEVEN])),
                               set![CARD_SPADES_QUEEN]);
        assert_eq!(valid_moves(negative_contract_move_validator,
                               &Hand::from_iter(cards.iter()),
                               &make_trick([CARD_SPADES_KING, CARD_SPADES_SEVEN])),
                               set![CARD_SPADES_EIGHT, CARD_SPADES_QUEEN]);
        assert_eq!(valid_moves(negative_contract_move_validator,
                               &Hand::from_iter(cards.iter()),
                               &make_trick([CARD_SPADES_KING, CARD_SPADES_SEVEN, CARD_TAROCK_2])),
                               set![CARD_SPADES_EIGHT, CARD_SPADES_QUEEN]);
    }

    #[test]
    fn negative_contract_pagat_can_only_be_played_as_last_tarock() {
        let cards = set![CARD_TAROCK_13, CARD_HEARTS_JACK, CARD_TAROCK_PAGAT, CARD_TAROCK_5];
        assert_eq!(valid_moves(negative_contract_move_validator,
                               &Hand::from_iter(cards.iter()),
                               &make_trick([CARD_TAROCK_12, CARD_SPADES_SEVEN])),
                               set![CARD_TAROCK_13]);
        assert_eq!(valid_moves(negative_contract_move_validator,
                               &Hand::from_iter(cards.iter()),
                               &make_trick([CARD_TAROCK_SKIS, CARD_SPADES_SEVEN])),
                               set![CARD_TAROCK_13, CARD_TAROCK_5]);
        assert_eq!(valid_moves(negative_contract_move_validator,
                               &Hand::from_iter(cards.iter()),
                               &make_trick([CARD_SPADES_QUEEN])),
                               set![CARD_TAROCK_13, CARD_TAROCK_5]);
    }

    #[test]
    fn negative_contract_pagat_as_not_last_tarock_must_be_played_if_trula_wins_trick() {
        let cards = set![CARD_TAROCK_13, CARD_HEARTS_JACK, CARD_TAROCK_PAGAT, CARD_TAROCK_5];
        assert_eq!(valid_moves(negative_contract_move_validator,
                               &Hand::from_iter(cards.iter()),
                               &make_trick([CARD_TAROCK_12, CARD_TAROCK_SKIS, CARD_TAROCK_MOND])),
                               set![CARD_TAROCK_PAGAT]);
        assert_eq!(valid_moves(negative_contract_move_validator,
                               &Hand::from_iter(cards.iter()),
                               &make_trick([CARD_TAROCK_SKIS, CARD_DIAMONDS_JACK, CARD_TAROCK_MOND])),
                               set![CARD_TAROCK_PAGAT]);
    }
}
