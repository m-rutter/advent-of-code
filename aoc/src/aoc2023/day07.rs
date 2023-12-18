use std::{cmp::Ordering, collections::HashMap, str::FromStr};

use anyhow::anyhow;
use itertools::Itertools;

use crate::{
    error::{AoCError, Result},
    Solution,
};

pub fn run(input: &str) -> Result<Solution> {
    let iter = input
        .trim()
        .lines()
        .map(|line| line.trim().parse::<CardWithBet>());

    let mut hands_without_joker: Vec<HandWithBet> = iter
        .clone()
        .map_ok(|cards| HandWithBet::new(cards, false))
        .collect::<Result<_>>()?;

    hands_without_joker.sort();

    let part_one = hands_without_joker
        .iter()
        .enumerate()
        .map(|(rank, hand)| hand.bet * (rank as u64 + 1))
        .sum::<u64>()
        .to_string();

    let mut hands_with_joker: Vec<HandWithBet> = iter
        .clone()
        .map_ok(|cards| HandWithBet::new(cards, true))
        .collect::<Result<_>>()?;

    hands_with_joker.sort();

    let part_two = hands_with_joker
        .iter()
        .enumerate()
        .map(|(rank, hand)| hand.bet * (rank as u64 + 1))
        .sum::<u64>()
        .to_string();

    Ok(Solution { part_one, part_two })
}

#[derive(Debug)]
struct CardWithBet {
    cards: [Card; 5],
    bet: u64,
}

#[derive(Debug, PartialEq, Eq, Ord)]
struct HandWithBet {
    cards: [Card; 5],
    hand: Hand,
    bet: u64,
    joker_rule: bool,
}

impl PartialOrd for HandWithBet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.hand.cmp(&other.hand) {
            Ordering::Equal => {
                for (self_card, other_card) in self.cards.iter().zip(other.cards.iter()) {
                    if self.joker_rule {
                        if self_card == &Card::Jack && other_card != &Card::Jack {
                            return Some(Ordering::Less);
                        }

                        if self_card != &Card::Jack && other_card == &Card::Jack {
                            return Some(Ordering::Greater);
                        }
                    }

                    match self_card.cmp(other_card) {
                        Ordering::Equal => continue,
                        other => return Some(other),
                    }
                }

                Some(Ordering::Equal)
            }
            other => Some(other),
        }
    }
}

impl CardWithBet {
    fn new(cards: [Card; 5], bet: u64) -> Self {
        CardWithBet { bet, cards }
    }
}

impl HandWithBet {
    fn new(cards_and_bet: CardWithBet, joker_rule: bool) -> Self {
        HandWithBet {
            bet: cards_and_bet.bet,
            hand: HandWithBet::hand_type(&cards_and_bet.cards, joker_rule),
            cards: cards_and_bet.cards,
            joker_rule,
        }
    }

    fn hand_type(cards: &[Card; 5], joker_rule: bool) -> Hand {
        let counts: HashMap<&Card, u8> = cards.iter().fold(HashMap::new(), |mut acc, card| {
            if joker_rule && card == &Card::Jack {
                return acc;
            }

            let count = acc.entry(&card).or_insert(0);
            *count += 1;

            acc
        });

        let mut best_hand = match counts.iter().map(|(_, c)| c).max().unwrap_or(&0) {
            // joker rule must be in effect and all of the cards are jacks/jokers
            0 => Hand::FiveOfAKind,
            1 => Hand::HighCard,
            2 => {
                if counts.iter().filter(|(_, c)| **c == 2).count() == 2 {
                    Hand::TwoPair
                } else {
                    Hand::OnePair
                }
            }
            3 => {
                if counts.iter().filter(|(_, c)| **c >= 2).count() == 2 {
                    Hand::FullHouse
                } else {
                    Hand::ThreeOfAKind
                }
            }

            4 => Hand::FourOfAKind,
            5 => Hand::FiveOfAKind,
            _ => unreachable!(),
        };

        if joker_rule {
            let joker_count = cards.iter().filter(|c| *c == &Card::Jack).count();

            for _ in 0..joker_count {
                best_hand.promote_with_joker_rule();
            }
        }

        best_hand
    }
}

impl FromStr for CardWithBet {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self> {
        let mut parts = s.split_whitespace();

        let first = parts.next().ok_or(anyhow!("expected hand of cards"))?;
        let last = parts.last().ok_or(anyhow!("expected bet"))?;

        let cards: Vec<Card> = first
            .chars()
            .filter(|c| !c.is_whitespace())
            .map(|c| Card::try_from(&c))
            .collect::<Result<_>>()?;

        let bet: u64 = last.parse()?;

        let cards = {
            if cards.len() != 5 {
                return Err(anyhow!("expected 5 cards, got {}", cards.len()))?;
            }
            let mut iter = cards.into_iter();

            [(); 5].map(|_| iter.next().unwrap())
        };

        Ok(CardWithBet::new(cards, bet))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl TryFrom<&char> for Card {
    type Error = AoCError;

    fn try_from(s: &char) -> Result<Self> {
        Ok(match s {
            'A' => Card::Ace,
            'K' => Card::King,
            'Q' => Card::Queen,
            'J' => Card::Jack,
            'T' => Card::Ten,
            '9' => Card::Nine,
            '8' => Card::Eight,
            '7' => Card::Seven,
            '6' => Card::Six,
            '5' => Card::Five,
            '4' => Card::Four,
            '3' => Card::Three,
            '2' => Card::Two,
            _ => Err(anyhow!("invalid card {s}"))?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Hand {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Hand {
    fn promote_with_joker_rule(&mut self) {
        *self = match self {
            Hand::HighCard => Hand::OnePair,
            Hand::OnePair => Hand::ThreeOfAKind,
            Hand::TwoPair => Hand::FullHouse,
            // the full house case is a fiction - its not possible to start from a real full house
            // and get a better hand with the joker rule. We must be in the middle of promoting
            // from a original starting point of TwoPair
            Hand::FullHouse | Hand::ThreeOfAKind => Hand::FourOfAKind,
            Hand::FourOfAKind | Hand::FiveOfAKind => Hand::FiveOfAKind,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_offical_example() {
        let input = r"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        let solution = run(input).unwrap();

        assert_eq!(solution.part_one, "6440");
        assert_eq!(solution.part_two, "5905");
    }

    #[test]
    fn matches_offical_input() {
        let input = include_str!("./input/day07");

        let solution = run(input).unwrap();

        assert_eq!(solution.part_one, "251287184");
        assert_eq!(solution.part_two, "250757288");
    }
}
