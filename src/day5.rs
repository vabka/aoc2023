use std::io::{BufRead, BufReader, Read};
use std::ops::{Add, Sub};
use std::str::FromStr;
use crate::Id;

pub fn a(reader: &mut impl Read) -> Option<u64> {
    let mut lines = BufReader::new(reader).lines().map(Result::ok).filter_map(Id::id)
        // .inspect(|line| println!("> {line}"))
        ;

    parse_almanac(&mut lines)
        .and_then(|almanac| {
            almanac.seeds.iter().map(|seed| almanac.get_location_by_seed(*seed)).min()
        })
        .map(Into::into)
}

fn parse_almanac(lines: &mut impl Iterator<Item=String>) -> Option<Almanac> {
    let seeds_line = lines.next()?;
    let seeds: Vec<_> = {
        let mut parts = seeds_line.splitn(2, ':');
        let keyword = parts.next()?;
        if keyword != "seeds" {
            eprintln!("Unexpected keyword: {keyword}. Expected 'seeds'");
            return None;
        }
        parts.next()?.trim()
            .split(' ')
            .map(str::trim)
            .filter_map(|x| u64::from_str(x).ok())
            .map(Into::into)
            .collect()
    };
    {
        let empty_line = lines.next()?;
        if empty_line.trim() != "" {
            return None;
        }
    }
    fn parse_mapping<From: std::convert::From<u64>, To: std::convert::From<u64>>(lines: &mut impl Iterator<Item=String>, expected_header: &str) -> Option<Mapping<From, To>> {
        let header = lines.next()?;
        if header.trim() != expected_header {
            eprintln!("Unexpected mapping header: {header}. Expected '{expected_header}'");
            return None;
        }
        let mut ranges = vec![];
        loop {
            let Some(next_line) = lines.next() else {
                break;
            };

            let trimmed = next_line.trim();
            if trimmed == "" {
                break;
            }
            let mut numbers = trimmed.splitn(3, ' ')
                .map(str::trim)
                .filter_map(|x| u64::from_str(x).ok());
            let range = MappingRange {
                destination_start: numbers.next()?.into(),
                source_start: numbers.next()?.into(),
                length: numbers.next()? as usize, // TODO
            };
            ranges.push(range)
        }
        Some(Mapping {
            ranges
        })
    }
    let almanac = Almanac::new(
        seeds,
        parse_mapping(lines, "seed-to-soil map:")?,
        parse_mapping(lines, "soil-to-fertilizer map:")?,
        parse_mapping(lines, "fertilizer-to-water map:")?,
        parse_mapping(lines, "water-to-light map:")?,
        parse_mapping(lines, "light-to-temperature map:")?,
        parse_mapping(lines, "temperature-to-humidity map:")?,
        parse_mapping(lines, "humidity-to-location map:")?,
    );
    Some(almanac)
}


macro_rules! new_type {
    ($wrap: ty, $name: ident) => {
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash)]
        struct $name($wrap);

        impl From<$wrap> for $name {
            fn from(value: u64) -> Self {
                Self(value)
            }
        }

        impl From<$name> for usize {
            fn from(value: $name) -> Self {
                value.0 as usize
            }
        }

        impl From<$name> for $wrap {
            fn from(value: $name) -> Self {
                value.0
            }
        }

        impl Add<usize> for $name {
            type Output = Self;

            fn add(self, rhs: usize) -> Self::Output {
                Self(self.0 + rhs as $wrap)
            }
        }

        impl Sub<$wrap> for $name {
            type Output = Self;

            fn sub(self, rhs: $wrap) -> Self::Output {
                Self(self.0 - rhs)
            }
        }

        impl Sub for $name {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self::Output {
                self - rhs.0
            }
        }
    };
    ($wrap: ty, $name: ident, $to: ident) => {
        new_type!($wrap, $name);
        impl From<$name> for $to {
            fn from(value: $name) -> Self {
                $to::from(value.0)
            }
        }
    };
}

new_type!(u64, Seed, Soil);
new_type!(u64, Soil, Fertilizer);
new_type!(u64, Fertilizer, Water);
new_type!(u64, Water, Light);
new_type!(u64, Light, Temperature);
new_type!(u64, Temperature, Humidity);
new_type!(u64, Humidity, Location);
new_type!(u64, Location);

struct Almanac {
    seeds: Vec<Seed>,
    seed_to_soil: Mapping<Seed, Soil>,
    // seed_to_soil_cache: HashMap<u64, u64>,

    soil_to_fertilizer: Mapping<Soil, Fertilizer>,
    // soil_to_fertilizer_cache: HashMap<u64, u64>,

    fertilizer_to_water: Mapping<Fertilizer, Water>,
    // fertilizer_to_water_cache: HashMap<u64, u64>,
    water_to_light: Mapping<Water, Light>,
    light_to_temperature: Mapping<Light, Temperature>,
    temperature_to_humidity: Mapping<Temperature, Humidity>,
    humidity_to_location: Mapping<Humidity, Location>,
}

impl Almanac {
    fn new(seeds: Vec<Seed>,
           seed_to_soil: Mapping<Seed, Soil>,
           soil_to_fertilizer: Mapping<Soil, Fertilizer>,
           fertilizer_to_water: Mapping<Fertilizer, Water>,
           water_to_light: Mapping<Water, Light>,
           light_to_temperature: Mapping<Light, Temperature>,
           temperature_to_humidity: Mapping<Temperature, Humidity>,
           humidity_to_location: Mapping<Humidity, Location>,
    ) -> Self {
        Self {
            seeds,
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        }
    }

    fn get_location_by_seed(&self, seed: Seed) -> Location {
        let soil = self.seed_to_soil.map(seed);
        let fertilizer = self.soil_to_fertilizer.map(soil);
        let water = self.fertilizer_to_water.map(fertilizer);
        let light = self.water_to_light.map(water);
        let temperature = self.light_to_temperature.map(light);
        let humidity = self.temperature_to_humidity.map(temperature);
        self.humidity_to_location.map(humidity)
    }
}

struct Mapping<From, To> {
    ranges: Vec<MappingRange<From, To>>,
}

impl<From: Copy, To> Mapping<From, To>
    where
        From: Ord + Add<usize, Output=From> + Sub<u64, Output=From> + Sub<From, Output=From> + Into<usize> + Into<To> + Copy,
        To: Add<usize, Output=To> + Copy
{
    fn map(&self, value: From) -> To {
        for range in &self.ranges {
            if let Some(value) = range.map(value) {
                return value;
            }
        }
        value.into()
    }
}

struct MappingRange<From, To> {
    source_start: From,
    destination_start: To,
    length: usize,
}

impl<'a, From, To> MappingRange<From, To>
    where
        From: Ord + Add<usize, Output=From> + Sub<u64, Output=From> + Sub<From, Output=From> + Into<usize> + Copy,
        To: Add<usize, Output=To> + Copy

{
    fn map(&self, value: From) -> Option<To> {
        if value < self.source_start {
            return None;
        }

        if value > (self.source_start + self.length - 1) {
            return None;
        }

        let offset = (value - self.source_start).into();
        Some(self.destination_start + offset)
    }
}