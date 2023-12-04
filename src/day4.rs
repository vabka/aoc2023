use std::collections::{HashMap, HashSet};
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

pub fn b(reader: &mut impl Read) -> u128 {
    struct E {
        matches: usize,
        count: u64,
    }
    let mut tickets: HashMap<usize, E> = HashMap::new();
    let mut total_count = 0u128;
    for line in BufReader::new(reader).lines().map(Result::ok).filter_map(Id::id) {
        let Some((ticket_id, matches)) = matches(&line) else {
            // println!("ERROR: {line}");
            continue;
        };
        let (count, matches) = {
            let entry = tickets.entry(ticket_id)
                .and_modify(|x| {
                    x.count += 1;
                    x.matches = matches;
                })
                .or_insert_with(|| E { matches, count: 1 });
            let count = entry.count;
            let matches = entry.matches;
            (count, matches)
        };
        // println!("{line} has {matches} matches. We have {count} copies of it.");
        for i in 1..=matches {
            // println!("So we add {count} copies of ticket {}.", ticket_id + i);
            tickets.entry(ticket_id + i)
                .and_modify(|x| x.count += count)
                .or_insert_with(|| E { count, matches: 0 });
        }
        total_count += count as u128;
    }
    total_count
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

fn matches(line: &str) -> Option<(usize, usize)> {
    let mut parts = line.splitn(2, ':').map(str::trim);
    let number = parts.next()?
        .splitn(2, ' ')
        .map(str::trim)
        .skip(1)
        .next().and_then(|x| usize::from_str(x).ok())?;
    let mut parts = parts.next()?.split('|');
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
    let result = (number, winning_numbers.intersection(&actual_numbers).count());
    Some(result)
}