use std::fmt;
use std::fmt::{Formatter, Show};
use std::hash::Hash;
use std::iter::AdditiveIterator;

use std::collections::HashSet;
use std::collections::hashmap::SetItems;
use std::rand::Rng;

use contracts::ContractType;

#[deriving(Clone, Show, Eq, PartialEq, Hash)]
pub enum CardSuit {
    Clubs,
    Spades,
    Hearts,
    Diamonds,
}

#[deriving(Clone, Show, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum CardRank {
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Knight,
    Queen,
    King,
}

#[deriving(Clone, Show, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Tarock {
    Tarock1,
    Tarock2,
    Tarock3,
    Tarock4,
    Tarock5,
    Tarock6,
    Tarock7,
    Tarock8,
    Tarock9,
    Tarock10,
    Tarock11,
    Tarock12,
    Tarock13,
    Tarock14,
    Tarock15,
    Tarock16,
    Tarock17,
    Tarock18,
    Tarock19,
    Tarock20,
    Tarock21,
    TarockSkis,
}

#[deriving(Clone, Show, Eq, PartialEq, Hash)]
pub enum Card {
    TarockCard(Tarock),
    SuitCard(CardRank, CardSuit),
}

impl Card {
    pub fn is_tarock(&self) -> bool {
        match self {
            &TarockCard(_) => true,
            _ => false
        }
    }

    pub fn is_pagat(&self) -> bool {
        match *self {
            TarockCard(Tarock1) => true,
            _ => false,
        }
    }

    pub fn is_mond(&self) -> bool {
        match *self {
            TarockCard(Tarock21) => true,
            _ => false,
        }
    }

    pub fn is_skis(&self) -> bool {
        match *self {
            TarockCard(TarockSkis) => true,
            _ => false,
        }
    }

    pub fn is_valuable(&self) -> bool {
        self.value() > 0
    }

    pub fn is_empty(&self) -> bool {
        !self.is_valuable()
    }

    pub fn suit(&self) -> Option<CardSuit> {
        match *self {
            SuitCard(_, suit) => Some(suit),
            _ => None,
        }
    }

    pub fn value(&self) -> uint {
        match *self {
            SuitCard(rank, _) => {
                match rank {
                    King  => 5,
                    Queen => 4,
                    Knight => 3,
                    Jack => 2,
                    _ => 0
                }
            }
            TarockCard(tarock) => {
                match tarock {
                    Tarock1 | Tarock21 | TarockSkis => 5,
                    _ => 0
                }
            }
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Card) -> Option<Ordering> {
        match (*self, *other) {
            (TarockCard(_), SuitCard(_, _)) => Some(Greater),
            (SuitCard(_, _), TarockCard(_)) => Some(Less),
            (SuitCard(rank, suit), SuitCard(rank_other, suit_other)) => {
                if suit == suit_other {
                    rank.partial_cmp(&rank_other)
                } else {
                    Some(Greater)
                }
            }
            (TarockCard(tarock), TarockCard(tarock_other)) => {
                tarock.partial_cmp(&tarock_other)
            }
        }
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Card) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub const CARD_CLUBS_SEVEN: Card = SuitCard(Seven, Clubs);
pub const CARD_CLUBS_EIGHT: Card = SuitCard(Eight, Clubs);
pub const CARD_CLUBS_NINE: Card = SuitCard(Nine, Clubs);
pub const CARD_CLUBS_TEN: Card = SuitCard(Ten, Clubs);
pub const CARD_CLUBS_JACK: Card = SuitCard(Jack, Clubs);
pub const CARD_CLUBS_KNIGHT: Card = SuitCard(Knight, Clubs);
pub const CARD_CLUBS_QUEEN: Card = SuitCard(Queen, Clubs);
pub const CARD_CLUBS_KING: Card = SuitCard(King, Clubs);

pub const CARD_SPADES_SEVEN: Card = SuitCard(Seven, Spades);
pub const CARD_SPADES_EIGHT: Card = SuitCard(Eight, Spades);
pub const CARD_SPADES_NINE: Card = SuitCard(Nine, Spades);
pub const CARD_SPADES_TEN: Card = SuitCard(Ten, Spades);
pub const CARD_SPADES_JACK: Card = SuitCard(Jack, Spades);
pub const CARD_SPADES_KNIGHT: Card = SuitCard(Knight, Spades);
pub const CARD_SPADES_QUEEN: Card = SuitCard(Queen, Spades);
pub const CARD_SPADES_KING: Card = SuitCard(King, Spades);

pub const CARD_HEARTS_SEVEN: Card = SuitCard(Seven, Hearts);
pub const CARD_HEARTS_EIGHT: Card = SuitCard(Eight, Hearts);
pub const CARD_HEARTS_NINE: Card = SuitCard(Nine, Hearts);
pub const CARD_HEARTS_TEN: Card = SuitCard(Ten, Hearts);
pub const CARD_HEARTS_JACK: Card = SuitCard(Jack, Hearts);
pub const CARD_HEARTS_KNIGHT: Card = SuitCard(Knight, Hearts);
pub const CARD_HEARTS_QUEEN: Card = SuitCard(Queen, Hearts);
pub const CARD_HEARTS_KING: Card = SuitCard(King, Hearts);

pub const CARD_DIAMONDS_SEVEN: Card = SuitCard(Seven, Diamonds);
pub const CARD_DIAMONDS_EIGHT: Card = SuitCard(Eight, Diamonds);
pub const CARD_DIAMONDS_NINE: Card = SuitCard(Nine, Diamonds);
pub const CARD_DIAMONDS_TEN: Card = SuitCard(Ten, Diamonds);
pub const CARD_DIAMONDS_JACK: Card = SuitCard(Jack, Diamonds);
pub const CARD_DIAMONDS_KNIGHT: Card = SuitCard(Knight, Diamonds);
pub const CARD_DIAMONDS_QUEEN: Card = SuitCard(Queen, Diamonds);
pub const CARD_DIAMONDS_KING: Card = SuitCard(King, Diamonds);

pub const CARD_TAROCK_PAGAT: Card = TarockCard(Tarock1);
pub const CARD_TAROCK_2: Card = TarockCard(Tarock2);
pub const CARD_TAROCK_3: Card = TarockCard(Tarock3);
pub const CARD_TAROCK_4: Card = TarockCard(Tarock4);
pub const CARD_TAROCK_5: Card = TarockCard(Tarock5);
pub const CARD_TAROCK_6: Card = TarockCard(Tarock6);
pub const CARD_TAROCK_7: Card = TarockCard(Tarock7);
pub const CARD_TAROCK_8: Card = TarockCard(Tarock8);
pub const CARD_TAROCK_9: Card = TarockCard(Tarock9);
pub const CARD_TAROCK_10: Card = TarockCard(Tarock10);
pub const CARD_TAROCK_11: Card = TarockCard(Tarock11);
pub const CARD_TAROCK_12: Card = TarockCard(Tarock12);
pub const CARD_TAROCK_13: Card = TarockCard(Tarock13);
pub const CARD_TAROCK_14: Card = TarockCard(Tarock14);
pub const CARD_TAROCK_15: Card = TarockCard(Tarock15);
pub const CARD_TAROCK_16: Card = TarockCard(Tarock16);
pub const CARD_TAROCK_17: Card = TarockCard(Tarock17);
pub const CARD_TAROCK_18: Card = TarockCard(Tarock18);
pub const CARD_TAROCK_19: Card = TarockCard(Tarock19);
pub const CARD_TAROCK_20: Card = TarockCard(Tarock20);
pub const CARD_TAROCK_MOND: Card = TarockCard(Tarock21);
pub const CARD_TAROCK_SKIS: Card = TarockCard(TarockSkis);

pub static CARDS: [Card, ..54] = [
    CARD_CLUBS_SEVEN,
    CARD_CLUBS_EIGHT,
    CARD_CLUBS_NINE,
    CARD_CLUBS_TEN,
    CARD_CLUBS_JACK,
    CARD_CLUBS_KNIGHT,
    CARD_CLUBS_QUEEN,
    CARD_CLUBS_KING,
    CARD_SPADES_SEVEN,
    CARD_SPADES_EIGHT,
    CARD_SPADES_NINE,
    CARD_SPADES_TEN,
    CARD_SPADES_JACK,
    CARD_SPADES_KNIGHT,
    CARD_SPADES_QUEEN,
    CARD_SPADES_KING,
    CARD_HEARTS_SEVEN,
    CARD_HEARTS_EIGHT,
    CARD_HEARTS_NINE,
    CARD_HEARTS_TEN,
    CARD_HEARTS_JACK,
    CARD_HEARTS_KNIGHT,
    CARD_HEARTS_QUEEN,
    CARD_HEARTS_KING,
    CARD_DIAMONDS_SEVEN,
    CARD_DIAMONDS_EIGHT,
    CARD_DIAMONDS_NINE,
    CARD_DIAMONDS_TEN,
    CARD_DIAMONDS_JACK,
    CARD_DIAMONDS_KNIGHT,
    CARD_DIAMONDS_QUEEN,
    CARD_DIAMONDS_KING,
    CARD_TAROCK_PAGAT,
    CARD_TAROCK_2,
    CARD_TAROCK_3,
    CARD_TAROCK_4,
    CARD_TAROCK_5,
    CARD_TAROCK_6,
    CARD_TAROCK_7,
    CARD_TAROCK_8,
    CARD_TAROCK_9,
    CARD_TAROCK_10,
    CARD_TAROCK_11,
    CARD_TAROCK_12,
    CARD_TAROCK_13,
    CARD_TAROCK_14,
    CARD_TAROCK_15,
    CARD_TAROCK_16,
    CARD_TAROCK_17,
    CARD_TAROCK_18,
    CARD_TAROCK_19,
    CARD_TAROCK_20,
    CARD_TAROCK_MOND,
    CARD_TAROCK_SKIS,
];

pub struct Cards<'a> {
    iter: SetItems<'a, Card>,
}

impl<'a> Iterator<&'a Card> for Cards<'a> {
    fn next(&mut self) -> Option<&'a Card> {
        self.iter.next()
    }
}

#[deriving(Show, Eq, PartialEq, Clone)]
pub struct Hand {
    cards: HashSet<Card>,
}

impl Hand {
    pub fn empty() -> Hand {
        Hand{ cards: HashSet::new() }
    }

    pub fn new(cards: &[Card]) -> Hand {
        Hand::from_iter(cards.iter())
    }

    pub fn from_iter<'a, C: Iterator<&'a Card>>(cards: C) -> Hand {
        Hand{
            cards: cards.map(|c| *c).collect(),
        }
    }

    pub fn remove_card(&mut self, card: &Card) {
        self.cards.remove(card);
    }

    pub fn size(&self) -> uint {
        self.cards.len()
    }

    pub fn is_empty(&self) -> bool  {
        self.cards.is_empty()
    }

    pub fn has_tarock(&self) -> bool {
        self.cards.iter().any(|card| card.is_tarock())
    }

    pub fn has_suit(&self, suit: &CardSuit) -> bool {
        self.cards.iter().any(|card| card.suit() == Some(*suit))
    }

    pub fn has_card(&self, card: &Card) -> bool {
        self.cards.contains(card)
    }

    pub fn cards<'a>(&'a self) -> Cards<'a> {
        Cards {
            iter: self.cards.iter(),
        }
    }
}

pub struct Talon {
    cards: Vec<Card>,
}

impl Talon {
    fn new(cards: Vec<Card>) -> Talon {
        Talon {
            cards: cards,
        }
    }

    fn cards(&self) -> &[Card] {
        self.cards.as_slice()
    }

    fn size(&self) -> uint {
        self.cards.len()
    }
}

pub struct CardDeal {
    pub talon: Talon,
    pub hands: Vec<Hand>,
}

pub fn deal_four_player_standard(cards: &[Card]) -> CardDeal {
    const NUM_PLAYERS: uint = 4;

    let mut six_card_packets = cards.chunks(6);
    let talon = six_card_packets.next().unwrap();
    let mut hands = Vec::from_fn(NUM_PLAYERS, |_| {
        Hand::empty()
    });

    let mut player_index = 0;
    for packet in six_card_packets {
        insert_all(&mut hands.get_mut(player_index).cards, packet);
        player_index = (player_index + 1) % NUM_PLAYERS;
    }

    CardDeal {
        talon: Talon::new(talon.to_vec()),
        hands: hands
    }
}

fn insert_all<T: Eq + Hash + Clone>(set: &mut HashSet<T>, xs: &[T]) {
    for x in xs.iter() {
        set.insert(x.clone());
    }
}

#[deriving(Clone)]
pub struct Unshuffled;

#[deriving(Clone)]
pub struct Shuffled;

#[deriving(Clone)]
pub struct Deck<S> {
    cards: Vec<Card>,
}

impl<S> Show for Deck<S> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        self.cards.fmt(fmt)
    }
}

impl<S> Deck<S> {
    pub fn shuffle<R: Rng>(mut self, rng: &mut R) -> Deck<Shuffled> {
        rng.shuffle(self.cards.as_mut_slice());
        Deck {cards: self.cards}
    }

    pub fn size(&self) -> uint {
        self.cards.len()
    }
}

impl Deck<Unshuffled> {
    pub fn new() -> Deck<Unshuffled> {
        Deck{
            cards: CARDS.to_vec(),
        }
    }
}

impl Deck<Shuffled> {
    pub fn deal(&self, deal_strat: |&[Card]| -> CardDeal) -> CardDeal {
        deal_strat(self.cards.as_slice())
    }
}

pub struct TrickWinner {
    pub card_index: uint,
    pub card: Card,
}

pub struct Trick {
    cards: Vec<Card>,
}

impl Trick {
    pub fn empty() -> Trick {
        Trick {cards: Vec::new()}
    }

    pub fn new(card: Card) -> Trick {
        let mut trick = Trick::empty();
        trick.add_card(card);
        trick
    }

    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn clear(&mut self) {
        self.cards.clear()
    }

    pub fn count(&self) -> uint {
        self.cards.len()
    }

    pub fn is_empty(&self) -> bool {
        self.count() == 0
    }

    pub fn first(&self) -> Option<Card> {
        if self.cards.len() > 0 {
            Some(self.cards[0])
        } else {
            None
        }
    }

    pub fn cards(&self) -> &[Card] {
        self.cards.as_slice()
    }

    pub fn winner(&self, f: |&[Card]| -> uint) -> TrickWinner {
        let card_index = f(self.cards.as_slice());
        TrickWinner {
            card_index: card_index,
            card: self.cards[card_index],
        }
    }
}

#[deriving(Clone)]
pub struct Pile {
    cards: Vec<Card>,
}

impl Pile {
    pub fn new() -> Pile {
        Pile { cards: Vec::new() }
    }

    fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn add_trick(&mut self, trick: Trick) {
        for card in trick.cards.into_iter() {
            self.add_card(card)
        }
    }

    pub fn score(&self) -> uint {
        let mut total = 0;
        for group in self.cards.as_slice().chunks(3) {
            let score = group.iter().map(|c| c.value()).sum();
            let num_valuable = group.iter().filter(|c| c.is_valuable()).count();
            if group.len() > 1 {
                if score == 0 {
                    total += 1;
                } else {
                    total += score - (num_valuable - 1);
                }
            } else if num_valuable > 0 {
                total += score - 1;
            }
        }
        total
    }
}

#[cfg(test)]
mod test {
    use quickcheck::{Arbitrary, Gen};

    use std::collections::HashSet;
    use std::rand::{task_rng, Rng};

    use std::hash::Hash;
    use std::iter::AdditiveIterator;

    use super::*;

    impl Arbitrary for Deck<Shuffled> {
        fn arbitrary<G: Gen>(g: &mut G) -> Deck<Shuffled> {
            Deck::new().shuffle(g)
        }
    }

    impl Arbitrary for Deck<Unshuffled> {
        fn arbitrary<G: Gen>(_: &mut G) -> Deck<Unshuffled> {
            Deck::new()
        }
    }

    fn deck_to_piles<S, G: Rng>(g: &mut G, deck: Deck<S>) -> (Pile, Pile) {
        let split_at = g.gen::<uint>() % deck.size();
        let cards_one = deck.cards.as_slice().slice_to(split_at);
        let mut pile_one = Pile::new();
        for card in cards_one.iter() {
            pile_one.add_card(*card);
        }
        let cards_two = deck.cards.as_slice().slice_from(split_at);
        let mut pile_two = Pile::new();
        for card in cards_two.iter() {
            pile_two.add_card(*card);
        }
        (pile_one, pile_two)
    }

    fn insert_all<T: Clone + Hash + Eq>(set: &mut HashSet<T>, xs: &[T]) {
        for x in xs.iter() {
            set.insert(x.clone());
        }
    }

    #[test]
    fn tarock_cards_are_ordered() {
        assert_eq!(CARD_TAROCK_PAGAT.partial_cmp(&CARD_TAROCK_2), Some(Less));
        assert_eq!(CARD_TAROCK_2.partial_cmp(&CARD_TAROCK_PAGAT), Some(Greater));
    }

    #[test]
    fn suit_cards_are_ordered() {
        assert_eq!(CARD_HEARTS_QUEEN.partial_cmp(&CARD_HEARTS_KING), Some(Less));
        assert_eq!(CARD_HEARTS_KING.partial_cmp(&CARD_HEARTS_QUEEN), Some(Greater));
    }

    #[test]
    fn tarocks_are_greater_than_suit_cards() {
        assert_eq!(CARD_HEARTS_KING.partial_cmp(&CARD_TAROCK_PAGAT), Some(Less));
        assert_eq!(CARD_TAROCK_PAGAT.partial_cmp(&CARD_HEARTS_KING), Some(Greater));
    }

    #[test]
    fn first_card_of_different_suits_is_always_greater() {
        assert_eq!(CARD_HEARTS_SEVEN.partial_cmp(&CARD_SPADES_KING), Some(Greater));
        assert_eq!(CARD_SPADES_KING.partial_cmp(&CARD_HEARTS_SEVEN), Some(Greater));
    }

    #[test]
    fn new_card_deck_is_of_correct_size() {
        let deck = Deck::new();
        assert_eq!(deck.size(), 54);
    }

    #[quickcheck]
    fn shuffled_deck_has_the_same_cards(deck: Deck<Unshuffled>) -> bool {
        let mut card_set = HashSet::new();
        insert_all(&mut card_set, deck.cards.as_slice());
        let mut rng = task_rng();
        let shuffled = deck.shuffle(&mut rng);
        let mut shuffled_card_set = HashSet::new();
        insert_all(&mut shuffled_card_set, shuffled.cards.as_slice());
        card_set.len() == shuffled_card_set.len()
    }

    #[test]
    fn there_are_22_tarocks_in_a_deck() {
        let deck = Deck::new();
        assert_eq!(deck.cards.iter().filter(|c| c.is_tarock()).count(), 22);
    }

    #[test]
    fn there_are_19_valuable_cards_in_a_deck() {
        let num_valuable = Deck::new().cards.iter().filter(|c| c.is_valuable()).count();
        assert_eq!(num_valuable, 19);
    }

    #[test]
    fn there_are_35_empty_cards_in_a_deck() {
        let num_empty= Deck::new().cards.iter().filter(|c| c.is_empty()).count();
        assert_eq!(num_empty, 35);
    }

    #[test]
    fn there_are_four_player_hands_with_four_player_standard_deal_strategy() {
        let mut rng = task_rng();
        let dealt_cards = Deck::new().shuffle(&mut rng).deal(deal_four_player_standard);
        assert_eq!(dealt_cards.hands.len(), 4);
    }

    #[test]
    fn each_player_gets_twelve_cards_with_four_player_standard_deal_strategy() {
        let mut rng = task_rng();
        let dealt_cards = Deck::new().shuffle(&mut rng).deal(deal_four_player_standard);
        assert_eq!(dealt_cards.hands[0].size(), 12);
        assert_eq!(dealt_cards.hands[1].size(), 12);
        assert_eq!(dealt_cards.hands[2].size(), 12);
        assert_eq!(dealt_cards.hands[3].size(), 12);
    }

    #[quickcheck]
    fn all_cards_are_unique_with_four_player_standard_deal_strategy(deck: Deck<Shuffled>) -> bool {
        let num_cards_in_deck = Deck::new().cards.len();
        let dealt_cards = deck.deal(deal_four_player_standard);
        let mut card_set = HashSet::new();
        insert_all(&mut card_set, dealt_cards.talon.cards());
        for hand in dealt_cards.hands.iter() {
            let cards = hand.cards().map(|c| *c).collect::<Vec<_>>();
            insert_all(&mut card_set, cards.as_slice());
        }
        num_cards_in_deck == card_set.len()
    }

    #[quickcheck]
    fn all_cards_are_dealt_with_four_player_standard_deal_strategy(deck: Deck<Shuffled>) -> bool {
        let num_cards_in_deck = Deck::new().cards.len();
        let dealt_cards = deck.deal(deal_four_player_standard);
        let num_cards = dealt_cards.talon.size() +
            dealt_cards.hands.iter().map(|h| h.size()).sum();
        num_cards_in_deck == num_cards
    }

    #[test]
    fn total_score_of_a_deck_is_70() {
        let deck = Deck::new();
        let mut pile = Pile::new();
        for card in deck.cards.iter() {
            pile.add_card(*card);
        }
        assert_eq!(pile.score(), 70);
    }

    #[quickcheck]
    fn total_score_of_piles_is_always_the_same(deck: Deck<Shuffled>) -> bool {
        let mut rng = task_rng();
        let (pile_one, pile_two) = deck_to_piles(&mut rng, deck);
        pile_one.score() + pile_two.score() == 70
    }

    #[test]
    fn can_add_card_to_trick() {
        let mut trick = Trick::empty();
        assert_eq!(trick.count(), 0);
        trick.add_card(TarockCard(Tarock1));
        assert_eq!(trick.count(), 1);
        trick.add_card(TarockCard(Tarock2));
        assert_eq!(trick.count(), 2);
    }

    #[test]
    fn can_clear_trick_cards() {
        let mut trick = Trick::empty();
        trick.add_card(TarockCard(Tarock1));
        trick.add_card(TarockCard(Tarock2));
        assert_eq!(trick.count(), 2);
        trick.clear();
        assert_eq!(trick.count(), 0);
    }
}
