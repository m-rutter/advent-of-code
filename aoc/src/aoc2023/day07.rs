use std::{cmp::Ordering, collections::HashMap, str::FromStr};

use anyhow::anyhow;

use crate::{
    error::{AoCError, Result},
    Solution,
};

pub fn run(input: &str) -> Result<Solution> {
    let mut hands: Vec<HandsWithBet> = input
        .trim()
        .lines()
        .map(|line| line.trim().parse())
        .collect::<Result<_>>()?;

    hands.sort();

    let part_one = hands
        .iter()
        .enumerate()
        .map(|(rank, hand)| hand.bet * (rank as u64 + 1))
        .sum::<u64>()
        .to_string();

    Ok(Solution {
        part_one,
        part_two: "".to_string(),
    })
}

#[derive(Debug, PartialEq, Eq, Ord)]
struct HandsWithBet {
    cards: [Card; 5],
    hand: Hand,
    bet: u64,
}

impl PartialOrd for HandsWithBet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.hand.cmp(&other.hand) {
            Ordering::Equal => {
                for (self_card, other_card) in self.cards.iter().zip(other.cards.iter()) {
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

impl HandsWithBet {
    fn new(cards: [Card; 5], bet: u64) -> Self {
        HandsWithBet {
            bet,
            hand: HandsWithBet::hand_type(&cards),
            cards,
        }
    }

    fn hand_type(cards: &[Card; 5]) -> Hand {
        let counts: HashMap<&Card, u8> = cards.iter().fold(HashMap::new(), |mut acc, card| {
            let count = acc.entry(&card).or_insert(0);
            *count += 1;

            acc
        });

        match counts.len() {
            1 => Hand::FiveOfAKind,
            2 => {
                if counts.iter().any(|(_, c)| *c == 4) {
                    Hand::FourOfAKind
                } else {
                    Hand::FullHouse
                }
            }
            3 => {
                if counts.iter().any(|(_, c)| *c == 3) {
                    Hand::ThreeOfAKind
                } else {
                    Hand::TwoPair
                }
            }
            4 => Hand::OnePair,
            5 => Hand::HighCard,
            // there are only five cards, so this should be unreachable
            _ => unreachable!(),
        }
    }
}

impl FromStr for HandsWithBet {
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

        Ok(HandsWithBet::new(cards, bet))
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
    }

    #[test]
    fn matches_offical_input() {
        let input = include_str!("./input/day07");

        let solution = run(input).unwrap();

        assert_eq!(solution.part_one, "3760");
    }
}
