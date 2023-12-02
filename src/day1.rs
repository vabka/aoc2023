use std::io::{BufRead, BufReader, Read};

pub(crate) fn a(reader: &mut impl Read) -> u32 {
    BufReader::new(reader)
        .lines()
        .filter_map(Result::ok)
        .filter_map(|x| get_number(&x))
        .fold(0u32, |a, b| { a + b })
}

pub fn b(reader: &mut impl Read) -> u32 {
    BufReader::new(reader)
        .lines()
        .filter_map(Result::ok)
        .filter_map(|x| get_spelled_number(&x))
        .fold(0u32, |a, b| { a + b })
}

fn get_number(str: &str) -> Option<u32> {
    let digits = str.chars().filter_map(char_to_digit);
    let first = digits.clone().next().map(|x| x * 10);
    let last = digits.last();
    first.zip(last).map(|(a, b)| a + b)
}

fn get_spelled_number(str: &str) -> Option<u32> {
    let a = {
        let mut idx = str.len();
        let mut result: &'static str = "";
        for n in ACTUAL_NUMBERS {
            if let Some(start) = str.find(n) {
                if start < idx {
                    idx = start;
                    result = n;
                }
            }
        }
        number_to_digit(result).map(|x| x * 10)
    };
    let b = {
        let mut idx = 0usize;
        let mut result: &'static str = "";
        for n in ACTUAL_NUMBERS {
            if let Some(start) = str.rfind(n) {
                if start >= idx {
                    idx = start;
                    result = n;
                }
            }
        }
        number_to_digit(result).map(|x| x)
    };

    a.zip(b).map(|(a, b)| { a + b })
}

const ACTUAL_NUMBERS: [&'static str; 18] = [
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "1",
    "2",
    "3",
    "4",
    "5",
    "6",
    "7",
    "8",
    "9"
];

fn number_to_digit(str: &str) -> Option<u32> {
    match str {
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        "1" => Some(1),
        "2" => Some(2),
        "3" => Some(3),
        "4" => Some(4),
        "5" => Some(5),
        "6" => Some(6),
        "7" => Some(7),
        "8" => Some(8),
        "9" => Some(9),
        _ => None
    }
}

fn char_to_digit(c: char) -> Option<u32> {
    c.to_digit(10)
}

#[cfg(test)]
mod tests {
    use crate::day1::get_spelled_number;

    #[test]
    fn _33() {
        let str = "threerznlrhtkjp23mtflmbrzq395three";
        let r = get_spelled_number(str);
        assert_eq!(r, Some(33));
    }
}