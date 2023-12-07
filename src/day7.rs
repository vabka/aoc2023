use std::cmp::Ordering;
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read};
use std::str::FromStr;

pub(crate) fn a(reader: &mut impl Read) -> u32 {
    let mut game: Vec<_> = BufReader::new(reader).lines().flatten().filter_map(|s| parse_hand_with_bid(&s)).collect();
    game.sort_by(|a, b| a.0.cmp(&b.0));
    game.iter().map(|x| x.1).enumerate().map(|(a, b)| (a + 1) as u32 * b).fold(0, |a, b| a + b)
}

fn parse_hand_with_bid(str: &str) -> Option<(Hand, u32)> {
    let mut parts = str.splitn(2, ' ');
    let hand_part: Vec<_> = parts.next()?.chars().filter_map(|x| match x {
        '2' => Some(Card::Two),
        '3' => Some(Card::Three),
        '4' => Some(Card::Four),
        '5' => Some(Card::Five),
        '6' => Some(Card::Six),
        '7' => Some(Card::Seven),
        '8' => Some(Card::Eight),
        '9' => Some(Card::Nine),
        'T' => Some(Card::Ten),
        'J' => Some(Card::Jack),
        'Q' => Some(Card::Queen),
        'K' => Some(Card::King),
        'A' => Some(Card::Ace),
        _ => None
    }).collect();
    if hand_part.len() != 5 {
        return None;
    }
    let hand = Hand(hand_part);
    let bid = parts.next().map(|x| u32::from_str(x).ok()).flatten()?;
    Some((hand, bid))
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Ord, PartialOrd, Hash)]
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


#[derive(Clone, Eq, PartialEq, Debug)]
struct Hand(Vec<Card>);

#[derive(Copy, Clone, Eq, Ord, PartialOrd, PartialEq, Debug)]
enum Combo {
    OneHigh,
    Pair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[cfg(test)]
mod tests {
    use crate::day7::Combo;

    #[test]
    fn test() {
        assert!(Combo::OneHigh < Combo::FiveOfAKind);
    }
}

impl Hand {
    fn combo(&self) -> Combo {
        let hash_map = self.0.iter().fold(HashMap::new(), |mut a, b| {
            a.entry(b).and_modify(|x| *x += 1).or_insert(1);
            a
        });
        let mut pairs: Vec<(_, _)> = hash_map.iter().collect();
        pairs.sort_by(|(_, a), (_, b)| { b.cmp(a) });
        match pairs[0].1 {
            5 => Combo::FiveOfAKind,
            4 => Combo::FourOfAKind,
            3 => match pairs[1].1 {
                2 => Combo::FullHouse,
                1 => Combo::ThreeOfAKind,
                _ => unreachable!()
            },
            2 => match pairs[1].1 {
                2 => Combo::TwoPair,
                1 => Combo::Pair,
                _ => unreachable!()
            },
            1 => Combo::OneHigh,
            _ => unreachable!()
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.combo().cmp(&other.combo()) {
            Ordering::Equal => self.0.iter().zip(other.0.iter())
                .map(|(a, b)| a.cmp(b))
                .filter(|x| x != &Ordering::Equal)
                .next()
                .unwrap_or(Ordering::Equal),
            x => x
        }
    }
}