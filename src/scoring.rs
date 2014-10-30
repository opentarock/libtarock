use std::collections::HashMap;

use cards::{Pile, HALF_POINTS};
use contracts::{Klop};
use player::{PlayerId, ContractPlayers};

pub type PlayerScores = HashMap<PlayerId, int>;

pub fn score(players: &mut ContractPlayers) -> PlayerScores {
    if players.contract().is_klop() {
        score_klop(players)
    } else if players.contract().is_beggar() {
        score_beggar(players)
    } else {
        score_normal(players)
    }
}

fn score_normal(players: &mut ContractPlayers) -> PlayerScores {
    let contract = players.contract();
    let mut pile = Pile::new();
    let scoring = players.scoring_players();
    let mut p = Vec::with_capacity(2);
    // Add card piles of all scoring players to one pile.
    for player in scoring.into_iter() {
        p.push(player.id());
        pile.add_pile(player.take_pile());
    }
    // Score all the cards from the scoring players together.
    let score = pile.score();
    // Every scoring player gets the same amount of points.
    p.iter().map(|&player_id| {
        let score = score_sign(|| score > HALF_POINTS) * (score + contract.value());
        (player_id, round_score(score))
    }).collect()
}

fn score_sign(cond: || -> bool) -> int {
    if cond() {
        1
    } else {
        -1
    }
}

fn score_klop(players: &mut ContractPlayers) -> PlayerScores {
    let mut scores = HashMap::new();
    let scoring = players.scoring_players();
    // Cards are scored fore every player individually.
    for player in scoring.into_iter() {
        scores.insert(player.id(), -player.take_pile().score());
    }
    let winner_loser = scores.iter()
        .map(|(_, &score)| score)
        .find(|score| is_winner_loser(*score))
        .is_some();
    if !winner_loser {
        scores.iter().map(|(&player_id, &score)| (player_id, round_score(score))).collect()
    } else {
        // Set the max and -max scores for winner and loser respectively.
        scores.iter()
            .filter(|&(_, &score)| is_winner_loser(score))
            .map(|(&player_id, &score)| {
                let score = if is_winner(score) {
                    Klop.value()
                } else {
                    -Klop.value()
                };
                (player_id, score)
            })
            .collect()
    }
}

fn score_beggar(players: &mut ContractPlayers) -> PlayerScores {
    let contract = players.contract();
    let mut scores = HashMap::new();
    let scoring = players.scoring_players();
    assert!(scoring.len() == 1);
    let score = score_sign(|| scoring[0].pile().is_empty()) * contract.value();
    scores.insert(scoring[0].id(), score);
    scores
}

fn round_score(score: int) -> int {
    (score as f64 / 5.0).round() as int * 5
}

fn is_winner_loser(score: int) -> bool {
    is_winner(score) || is_loser(score)
}

fn is_winner(score: int) -> bool {
    score == 0
}

fn is_loser(score: int) -> bool {
    score < -HALF_POINTS
}

#[cfg(test)]
mod test {
    use cards::*;
    use contracts::{SoloWithout, Klop, Standard, Three, Two, Beggar, beggar};
    use player::{Players, PlayerId};

    use super::*;

    fn init_cards(players: &mut Players) {
        for card in [CARD_TAROCK_SKIS, CARD_CLUBS_EIGHT, CARD_HEARTS_JACK,
                     CARD_SPADES_QUEEN, CARD_TAROCK_14, CARD_HEARTS_KNIGHT].iter() {
            players.player_mut(2).pile_mut().add_card(*card);
        }
        players.player_mut(0).pile_mut().add_card(CARD_HEARTS_KING);
        players.player_mut(1).pile_mut().add_card(CARD_SPADES_KING);
        players.player_mut(1).pile_mut().add_card(CARD_SPADES_JACK);
        players.player_mut(3).pile_mut().add_card(CARD_DIAMONDS_KING);
    }

    fn init_no_cards(players: &mut Players, player: PlayerId) {
        *players.player_mut(player).pile_mut() = Pile::new();
    }

    fn init_half_points(players: &mut Players, player: PlayerId) {
        for card in [CARD_CLUBS_KING, CARD_CLUBS_QUEEN, CARD_CLUBS_KNIGHT,
                     CARD_TAROCK_SKIS, CARD_TAROCK_MOND, CARD_TAROCK_PAGAT,
                     CARD_HEARTS_KING, CARD_HEARTS_QUEEN, CARD_HEARTS_KNIGHT].iter() {
            players.player_mut(player).pile_mut().add_card(*card);
        }
    }

    #[test]
    fn score_for_declarer_is_calculated() {
        let mut players = Players::new(4);
        init_cards(&mut players);
        let mut cp = players.play_contract(2, SoloWithout);
        let scores = score(&mut cp);
        assert_eq!(scores.len(), 1);
        assert_eq!(scores[2], -90);
    }

    #[test]
    fn score_for_declarer_and_partner_is_calculated() {
        let mut players = Players::new(4);
        players.player_mut(3).set_partner(2);
        init_cards(&mut players);
        let mut cp = players.play_contract(3, Standard(Two));
        let scores = score(&mut cp);
        assert_eq!(scores.len(), 2);
        assert_eq!(scores[3], -35);
        assert_eq!(scores[3], scores[2]);
    }

    #[test]
    fn winning_the_contract_awards_positive_points() {
        let mut players = Players::new(4);
        players.player_mut(3).set_partner(2);
        init_cards(&mut players);
        init_half_points(&mut players, 2);
        let mut cp = players.play_contract(3, Standard(Three));
        let scores = score(&mut cp);
        assert_eq!(scores.len(), 2);
        assert_eq!(scores[3], 60);
        assert_eq!(scores[3], scores[2]);
    }

    #[test]
    fn every_player_is_scored_independently_in_klop() {
        let mut players = Players::new(4);
        init_cards(&mut players);
        let mut cp = players.play_contract(2, Klop);
        let scores = score(&mut cp);
        assert_eq!(scores.len(), 4);
        assert_eq!(scores[0], -5);
        assert_eq!(scores[1], -5);
        assert_eq!(scores[2], -10);
        assert_eq!(scores[3], -5);
    }

    #[test]
    fn klop_only_winner_scores() {
        let mut players = Players::new(4);
        init_cards(&mut players);
        init_no_cards(&mut players, 0);
        let mut cp = players.play_contract(2, Klop);
        let scores = score(&mut cp);
        assert_eq!(scores.len(), 1);
        assert_eq!(scores[0], 70);
    }

    #[test]
    fn klop_only_loser_scores() {
        let mut players = Players::new(4);
        init_cards(&mut players);
        init_half_points(&mut players, 1);
        let mut cp = players.play_contract(2, Klop);
        let scores = score(&mut cp);
        assert_eq!(scores.len(), 1);
        assert_eq!(scores[1], -70);
    }

    #[test]
    fn both_winner_and_loser_score() {
        let mut players = Players::new(4);
        init_cards(&mut players);
        init_no_cards(&mut players, 2);
        init_half_points(&mut players, 3);
        let mut cp = players.play_contract(0, Klop);
        let scores = score(&mut cp);
        assert_eq!(scores.len(), 2);
        assert_eq!(scores[2], 70);
        assert_eq!(scores[3], -70);
    }

    #[test]
    fn beggar_is_won_is_declarer_wins_no_tricks() {
        let mut players = Players::new(4);
        init_cards(&mut players);
        init_no_cards(&mut players, 2);
        let mut cp = players.play_contract(2, Beggar(beggar::Normal));
        let scores = score(&mut cp);
        assert_eq!(scores.len(), 1);
        assert_eq!(scores[2], 70);
    }

    #[test]
    fn beggar_is_lost_is_declarer_wins_no_tricks() {
        let mut players = Players::new(4);
        init_cards(&mut players);
        let mut cp = players.play_contract(2, Beggar(beggar::Open));
        let scores = score(&mut cp);
        assert_eq!(scores.len(), 1);
        assert_eq!(scores[2], -90);
    }
}
