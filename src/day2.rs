use std::io::{BufRead, BufReader, Read};
use std::str::FromStr;

pub(crate) fn a(reader: &mut impl Read) -> u32 {
    let setup = GameSetup {
        red: 12,
        green: 13,
        blue: 14,
    };
    BufReader::new(reader).lines().filter_map(Result::ok)
        .filter_map(|x| parse_game(&x))
        .filter(|game| game.verify(&setup))
        .fold(0u32, |a, b| { b.id + a })
}

fn parse_game(str: &str) -> Option<GameReport> {
    let mut parts = str.split(':').map(str::trim);
    let id = {
        let game_part = parts.next()?;
        game_part.split(' ').last().map(u32::from_str)?.ok()?
    };
    let sets = parts.next()?.split(';')
        .map(str::trim)
        .map(|s| {
            let cubes = s.split(',').map(str::trim);
            let mut blue = 0;
            let mut green = 0;
            let mut red = 0;
            for cubes_count in cubes {
                let mut parts = cubes_count.split(' ').map(str::trim);
                let count = parts.next().map(u32::from_str).map(Result::ok).flatten();
                let color = parts.next();
                if let Some((count, color)) = count.zip(color) {
                    match color {
                        "blue" => blue += count,
                        "green" => green += count,
                        "red" => red += count,
                        _ => {}
                    };
                }
            }
            GameSet { red, green, blue }
        }).collect();
    Some(GameReport {
        id,
        sets,
    })
}

struct GameSetup {
    red: u32,
    green: u32,
    blue: u32,
}

impl GameSetup {
    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

struct GameReport {
    id: u32,
    sets: Vec<GameSet>,
}

impl GameReport {
    fn verify(&self, setup: &GameSetup) -> bool {
        for set in &self.sets {
            if set.green > setup.green || set.blue > setup.blue || set.red > setup.red {
                return false;
            }
        }
        true
    }

    fn minimum_set(&self) -> GameSetup {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for set in &self.sets {
            if set.red > red {
                red = set.red;
            }
            if set.green > green {
                green = set.green;
            }
            if set.blue > blue {
                blue = set.blue;
            }
        }
        GameSetup {
            red,
            green,
            blue,
        }
    }
}

struct GameSet {
    red: u32,
    green: u32,
    blue: u32,
}

pub(crate) fn b(reader: &mut impl Read) -> u32 {
    BufReader::new(reader).lines().filter_map(Result::ok)
        .filter_map(|x| parse_game(&x))
        .map(|game| game.minimum_set().power())
        .fold(0u32, |a, b| { b + a })
}