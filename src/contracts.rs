use cards::{CardSuit, Card, TarockCard, Tarock1, Tarock21, TarockSkis};

pub enum ContractType {
    Three,
    Two,
    One,
}

pub mod beggar {
    pub enum Type {
        Normal,
        Open,
    }
}
pub mod valat {
    pub enum Type {
        Normal,
        Color,
    }
}

pub enum Contract {
    Klop,
    Standard(ContractType),
    Solo(ContractType),
    Beggar(beggar::Type),
    SoloWithout,
    Valat(valat::Type),
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
    }
    pagat && mond && skis
}

#[cfg(test)]
mod test {
    use cards::{SuitCard, TarockCard, King, Queen, Jack, Seven, Tarock1,
        Tarock6, Tarock9, Tarock12, Tarock20, Tarock21, TarockSkis, Hearts, Spades, Card};

    use super::{standard_winner_strategy, color_valat_winner_strategy};

    static HIGH_HEARTS_NO_TAROCKS: &'static [Card] = [
        SuitCard(Jack, Hearts),
        SuitCard(Queen, Hearts),
        SuitCard(King, Spades),
        SuitCard(Seven, Hearts),
    ];

    static SPADES: &'static [Card] = [
        SuitCard(King, Spades),
        SuitCard(Jack, Spades),
        SuitCard(Queen, Spades),
        SuitCard(Seven, Spades),
    ];

    static SPADES2: &'static [Card] = [
        SuitCard(Jack, Spades),
        SuitCard(Seven, Spades),
        SuitCard(King, Spades),
        SuitCard(Queen, Spades),
    ];

    static SUITS_WITH_TAROCK: &'static [Card] = [
        SuitCard(Jack, Hearts),
        SuitCard(King, Spades),
        SuitCard(Seven, Hearts),
        TarockCard(Tarock1),
    ];

    static JUST_TAROCKS: &'static [Card] = [
        TarockCard(Tarock9),
        TarockCard(Tarock12),
        TarockCard(Tarock20),
        TarockCard(Tarock6),
    ];

    static TAROCKS_TRULA: &'static [Card] = [
        TarockCard(TarockSkis),
        TarockCard(Tarock21),
        TarockCard(Tarock6),
        TarockCard(Tarock1),
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
}
