use std::{cmp::Ordering, collections::HashMap};

use crate::input_utils::read_input_lines;

// const LABELS1: [char; 13] = [
//     'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
// ];

const LABELS2: [char; 13] = [
    'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
];

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum HandType {
    FiveOfAKind = 0,
    FourOfAKind = 1,
    FullHouse = 2,
    ThreeOfAKind = 3,
    TwoPair = 4,
    OnePair = 5,
    HighCard = 6,
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &HandType) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HandType {
    fn cmp(&self, other: &HandType) -> Ordering {
        (*self as i64).cmp(&(*other as i64))
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: Vec<char>,
    bid: i64,
}

impl Hand {
    fn new(cards: Vec<char>, bid: i64) -> Hand {
        Hand { cards, bid }
    }

    fn get_type(&self) -> HandType {
        let mut map = HashMap::new();
        for card in self.cards.clone() {
            *map.entry(card).or_insert(0) += 1;
        }

        // Part 2 - BEGIN
        let mut highest_count = 0;
        let mut highest_card = '2';
        for (card, count) in map.iter() {
            if card != &'J' {
                if *count > highest_count {
                    highest_count = *count;
                    highest_card = *card;
                } else if *count == highest_count {
                    if LABELS2.iter().position(|&x| x == *card).unwrap()
                        < LABELS2.iter().position(|&x| x == highest_card).unwrap()
                    {
                        highest_card = *card;
                    }
                }
            }
        }

        if map.contains_key(&'J') {
            let j_count = map.get(&'J').unwrap().clone();
            if j_count < 5 {
                map.remove(&'J');
                if let Some(x) = map.get_mut(&highest_card) {
                    *x += j_count;
                } else {
                    println!("Error: highest card not found in map, {}", highest_card)
                }
            }
        }
        // Part 2 - END

        return match map.len() {
            1 => HandType::FiveOfAKind,
            2 => {
                if map.values().any(|&x| x == 4) {
                    HandType::FourOfAKind
                } else {
                    HandType::FullHouse
                }
            }
            3 => {
                if map.values().any(|&x| x == 3) {
                    HandType::ThreeOfAKind
                } else {
                    HandType::TwoPair
                }
            }
            4 => HandType::OnePair,
            _ => HandType::HighCard,
        };
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Hand) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Hand) -> Ordering {
        let a_type = self.get_type();
        let b_type = other.get_type();

        if a_type != b_type {
            return a_type.cmp(&b_type);
        }

        for i in 0..self.cards.len() {
            let a_card = self.cards[i];
            let b_card = other.cards[i];

            if a_card != b_card {
                return LABELS2
                    .iter()
                    .position(|&x| x == a_card)
                    .unwrap()
                    .cmp(&LABELS2.iter().position(|&x| x == b_card).unwrap());
            }
        }

        return Ordering::Equal;
    }
}

pub fn part1() {
    let input = read_input_lines("input/day7_part1.txt").expect("Could not read input file");

    let mut hands: Vec<Hand> = input
        .iter()
        .map(|line| {
            let mut line_iter = line.split_whitespace();
            let cards: Vec<char> = line_iter.next().unwrap().chars().collect();
            let bid = line_iter.next().unwrap().parse().unwrap();
            Hand::new(cards, bid)
        })
        .collect();

    hands.sort();

    let result: i64 = hands
        .iter()
        .rev()
        .enumerate()
        .map(|(index, hand)| (index + 1) as i64 * hand.bid)
        .sum();

    println!("Result: {:?}", result);
}

pub fn part2() {
    let input = read_input_lines("input/day7_part2.txt").expect("Could not read input file");

    let mut hands: Vec<Hand> = input
        .iter()
        .map(|line| {
            let mut line_iter = line.split_whitespace();
            let cards: Vec<char> = line_iter.next().unwrap().chars().collect();
            let bid = line_iter.next().unwrap().parse().unwrap();
            Hand::new(cards, bid)
        })
        .collect();

    hands.sort();

    let result: i64 = hands
        .iter()
        .rev()
        .enumerate()
        .map(|(index, hand)| (index + 1) as i64 * hand.bid)
        .sum();

    println!("Result: {:?}", result);
}
