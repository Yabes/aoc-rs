use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
enum CardLabel {
    J,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    T,
    Q,
    K,
    A,
}

impl CardLabel {
    fn from_char(c: char) -> Self {
        match c {
            'J' => CardLabel::J,
            '2' => CardLabel::Num2,
            '3' => CardLabel::Num3,
            '4' => CardLabel::Num4,
            '5' => CardLabel::Num5,
            '6' => CardLabel::Num6,
            '7' => CardLabel::Num7,
            '8' => CardLabel::Num8,
            '9' => CardLabel::Num9,
            'T' => CardLabel::T,
            'Q' => CardLabel::Q,
            'K' => CardLabel::K,
            'A' => CardLabel::A,
            _ => panic!("Unexpected char {}", c),
        }
    }
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
enum HandType {
    HighCard,     // [ 1, 1, 1, 1, 1 ]
    OnePair,      // [ 2, 1, 1, 1 ]
    TwoPairs,     // [ 2, 2, 1 ]
    ThreeOfAkind, // [ 3, 1, 1 ]
    FullHouse,    // [ 3, 2 ]
    FourOfAKind,  // [ 4, 1 ]
    FiveOfAKind,  // [ 5 ]
}

#[derive(Debug, Eq)]
struct Hand {
    raw_hand: String,
    bid: usize,
    hand_type: HandType,
}

impl Hand {
    fn from_str(text: &str) -> Result<Self, &'static str> {
        let mut spl = text.split(' ');

        let cards = spl.next().unwrap();
        let bid = spl.next().unwrap().parse::<usize>().unwrap();

        let mut map = HashMap::<char, i32>::new();
        for card in cards.chars() {
            map.entry(card).and_modify(|count| *count += 1).or_insert(1);
        }

        let mut joker_count = map.remove(&'J').unwrap_or(0);
        if map.is_empty() {
            map.insert('J', 5);
            joker_count = 0;
        }

        let mut a: Vec<i32> = map.into_values().collect();
        a.sort();
        a.reverse();

        let mut a = a.iter();
        let highest_card_count = a.next().unwrap() + joker_count;

        let hand_type = match highest_card_count {
            5 => HandType::FiveOfAKind,
            4 => HandType::FourOfAKind,
            3 => {
                let second_card_count = a.next().unwrap();

                match second_card_count {
                    2 => HandType::FullHouse,
                    1 => HandType::ThreeOfAkind,
                    _ => panic!("Unexpected card count: {}", second_card_count),
                }
            }
            2 => {
                let second_card_count = a.next().unwrap();

                match second_card_count {
                    2 => HandType::TwoPairs,
                    1 => HandType::OnePair,
                    _ => panic!("Unexpected card count: {}", second_card_count),
                }
            }
            1 => HandType::HighCard,
            _ => panic!("Unexpected card count: {}", highest_card_count),
        };

        Ok(Hand {
            bid,
            raw_hand: String::from(cards),
            hand_type,
        })
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Equal => {
                for (left, right) in self.raw_hand.chars().zip(other.raw_hand.chars()) {
                    let res = CardLabel::from_char(left).cmp(&CardLabel::from_char(right));

                    if res != Ordering::Equal {
                        return res;
                    }
                }

                Ordering::Equal
            }
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type
    }
}

pub fn part1(input: &str) -> Result<usize, &'static str> {
    let mut hands: Vec<Hand> = input.trim().lines().map(Hand::from_str).collect();
    hands.sort();
    Ok(hands
        .iter()
        .enumerate()
        .map(|(index, hand)| hand.bid * (index + 1))
        .sum())
}

pub fn part2(input: &str) -> Result<usize, &'static str> {
    let mut hands: Vec<Hand> = input.trim().lines().map(Hand::from_str).collect();
    hands.sort();

    Ok(hands
        .iter()
        .enumerate()
        .map(|(index, hand)| hand.bid * (index + 1))
        .sum())
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use super::*;

    const TEST_INPUT: &str = "
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";

    #[test]
    fn hand_type_ordering() {
        assert_eq!(
            HandType::FourOfAKind.cmp(&HandType::TwoPairs),
            Ordering::Greater,
        )
    }

    #[test]
    fn test_part1() {
        let expected = 6440;
        let result = part1(TEST_INPUT).unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn test_part2() {
        let expected = 5905;
        let result = part2(TEST_INPUT).unwrap();

        assert_eq!(expected, result);
    }
}
