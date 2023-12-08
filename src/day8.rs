use std::collections::{HashMap, HashSet};
use std::io::{BufRead, BufReader, Read};

pub(crate) fn a(reader: &mut impl Read) -> Option<u32> {
    let (instructions, map) = parse_map_with_instructions(reader)?;
    let mut current = "AAA";
    let mut iptr = 0usize;
    let mut steps = 0;
    while current != "ZZZ" {
        let (left, right) = map.get(current)?;
        let instruction = {
            if instructions.len() <= iptr {
                iptr = 0;
            }
            instructions[iptr]
        };
        current = match instruction {
            Turn::Left => left,
            Turn::Right => right
        };
        iptr += 1;
        steps += 1;
    }
    Some(steps)
}

pub fn b(reader: &mut impl Read) -> Option<u32> {
    let (instructions, map) = parse_map_with_instructions(reader)?;
    let mut cursors: Vec<_> = map.keys().filter(|x| x.ends_with('A')).map(|x| x.as_str()).collect();
    let mut cycles = Vec::with_capacity(cursors.len());
    cycles.resize(cursors.len(), 0);
    let mut ends = 0;
    let mut iptr = 0usize;
    let mut steps = 0;

    while ends != cursors.len() {
        let instruction = {
            if instructions.len() <= iptr {
                iptr = 0;
            }
            instructions[iptr]
        };
        ends = 0;
        for (i, cursor) in cursors.iter_mut().enumerate() {
            let (left, right) = map.get(*cursor)?;
            let old = *cursor;
            *cursor = match instruction {
                Turn::Left => left,
                Turn::Right => right
            };
            let new = *cursor;
            if cursor.ends_with('Z') {
                println!("{i} = {old} -> {new}");
                if cycles[i] == 0 {
                    cycles[i] = steps + 1;
                }
                ends += 1;
            }
        }
        iptr += 1;
        steps += 1;
        println!("{steps}. {ends} of {}", cursors.len());
        if cycles.iter().all(|x| *x > 0) {
            for cycle in cycles {
                println!("{cycle}");
            }
            return None;
        }
    }
    Some(steps)
}

fn parse_map_with_instructions(reader: &mut impl Read) -> Option<(Vec<Turn>, HashMap<String, (String, String)>)> {
    let mut lines = BufReader::new(reader).lines().flatten();
    let instructions: Vec<Turn> = lines.next()?.chars().filter_map(|x| match x {
        'L' => Some(Turn::Left),
        'R' => Some(Turn::Right),
        _ => None
    }).collect();
    let map = lines
        .filter_map(|l| {
            let s = l.trim();
            if s.len() == 0 {
                return None;
            }
            let mut parts = s.splitn(2, '=');
            let node = parts.next().map(str::trim)?.to_string();
            let mut rot = parts.next()?.trim().trim_start_matches('(').trim_end_matches(')').splitn(2, ',').map(str::trim);
            let l = rot.next()?.to_string();
            let r = rot.next()?.to_string();
            Some((node, l, r))
        })
        .fold(HashMap::new(), |mut a, b| {
            a.insert(b.0, (b.1, b.2));
            a
        });
    Some((instructions, map))
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Turn {
    Left,
    Right,
}