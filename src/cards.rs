use std::fmt;
use std::fmt::{Formatter, Show};
use std::iter::AdditiveIterator;

use std::rand::Rng;

#[deriving(Clone, Show, Eq, PartialEq, Hash)]
pub enum CardSuit {
    Clubs,
    Spades,
    Hearts,
    Diamonds,
}

static CARD_SUITS: [CardSuit, ..4] = [
    Clubs,
    Spades,
    Hearts,
    Diamonds,
];

#[deriving(Clone, Show, Eq, PartialEq, Hash)]
pub enum CardRank {
    King,
    Queen,
    Knight,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
}

static CARD_RANKS: [CardRank, ..8] = [
    King,
    Queen,
    Knight,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
];

#[deriving(Clone, Show, Eq, PartialEq, Hash)]
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

static TAROCKS: [Tarock, ..22] = [
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
];

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

    pub fn is_valuable(&self) -> bool {
        self.value() > 0
    }

    pub fn is_empty(&self) -> bool {
        !self.is_valuable()
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

pub struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    pub fn size(&self) -> uint {
        self.cards.len()
    }
}

pub struct CardDeal {
    talon: Vec<Card>,
    hands: Vec<Hand>,
}

pub fn deal_four_player_standard(cards: &[Card]) -> CardDeal {
    static NUM_PLAYERS: uint = 4;
    static CARDS_PER_PLAYER: uint = 12;

    let mut six_card_packets = cards.chunks(6);
    let talon = six_card_packets.next().unwrap();
    let mut hands = Vec::from_fn(NUM_PLAYERS, |_| {
        Hand {cards: Vec::with_capacity(CARDS_PER_PLAYER)}
    });

    let mut player_index = 0;
    for packet in six_card_packets {
        hands.get_mut(player_index).cards.push_all(packet);
        player_index = (player_index + 1) % NUM_PLAYERS;
    }

    CardDeal {
        talon: Vec::from_slice(talon),
        hands: hands
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
        let mut cards = Vec::new();
        for tarock in TAROCKS.iter() {
            cards.push(TarockCard(*tarock));
        }
        for suit in CARD_SUITS.iter() {
            for rank in CARD_RANKS.iter() {
                cards.push(SuitCard(*rank, *suit));
            }
        }
        Deck {cards: cards}
    }
}

impl Deck<Shuffled> {
    pub fn deal(&self, deal_strat: |&[Card]| -> CardDeal) -> CardDeal {
        deal_strat(self.cards.as_slice())
    }
}

pub struct TrickWinner {
    pub order_id: uint,
    pub card: Card,
}

pub trait TrickMaxStrategy {
    fn max(&self, &[Card]) -> uint;
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

    fn winner<S: TrickMaxStrategy>(&self, f: S) -> TrickWinner {
        let order_id = f.max(self.cards.as_slice());
        TrickWinner {
            order_id: order_id,
            card: self.cards[order_id],
        }
    }
}

pub struct Pile {
    cards: Vec<Card>,
}

impl Pile {
    pub fn new() -> Pile {
        Pile {cards: Vec::new()}
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

    use super::{TarockCard, Tarock1, Tarock2};
    use super::{Deck, Shuffled, Unshuffled, Pile, Trick, deal_four_player_standard};

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
        assert_eq!(dealt_cards.hands.get(0).size(), 12);
        assert_eq!(dealt_cards.hands.get(1).size(), 12);
        assert_eq!(dealt_cards.hands.get(2).size(), 12);
        assert_eq!(dealt_cards.hands.get(3).size(), 12);
    }

    #[quickcheck]
    fn all_cards_are_unique_with_four_player_standard_deal_strategy(deck: Deck<Shuffled>) -> bool {
        let num_cards_in_deck = Deck::new().cards.len();
        let dealt_cards = deck.deal(deal_four_player_standard);
        let mut card_set = HashSet::new();
        insert_all(&mut card_set, dealt_cards.talon.as_slice());
        for hand in dealt_cards.hands.iter() {
            insert_all(&mut card_set, hand.cards.as_slice());
        }
        num_cards_in_deck == card_set.len()
    }

    #[quickcheck]
    fn all_cards_are_dealt_with_four_player_standard_deal_strategy(deck: Deck<Shuffled>) -> bool {
        let num_cards_in_deck = Deck::new().cards.len();
        let dealt_cards = deck.deal(deal_four_player_standard);
        let num_cards = dealt_cards.talon.len() +
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
