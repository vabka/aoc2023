use std::collections::HashSet;
use std::io::{BufRead, BufReader, Read};
use std::str::FromStr;
use crate::Id;

pub(crate) fn a(reader: &mut impl Read) -> u32 {
    BufReader::new(reader).lines().map(Result::ok).filter_map(Id::id).filter_map(|l| numbers(&l))
        .map(|(w, a)|
            match w.intersection(&a).count() {
                0 => 0,
                1 => 1,
                x => 1 << (x - 1)
            }
        )
        .fold(0, |a, b| a + b)
}

fn numbers(line: &str) -> Option<(HashSet<u32>, HashSet<u32>)> {
    let mut parts = line.split(':').skip(1).next()?.split('|');
    let winning_numbers: HashSet<_> = parts.next()?
        .split(' ')
        .map(str::trim)
        .filter_map(|x| u32::from_str(x).ok())
        .collect();
    let actual_numbers: HashSet<_> = parts.next()?
        .split(' ')
        .map(str::trim)
        .filter_map(|x| u32::from_str(x).ok())
        .collect();
    Some((winning_numbers, actual_numbers))
}