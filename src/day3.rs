use std::collections::HashSet;
use std::io::{BufRead, BufReader, Read};

pub(crate) fn a(reader: &mut impl Read) -> u32 {
    read_schematic(reader)
        .iter_parts()
        .fold(0, |a, b| a + b.part_no.iter().fold(0, |a, b| a + b.number))
}


pub(crate) fn b(reader: &mut impl Read) -> u32 {
    read_schematic(reader)
        .iter_parts()
        .filter(|x| x.symbol == '*' && x.part_no.len() == 2)
        .map(|x| x.part_no.iter().fold(1, |a, b| a * b.number))
        .fold(0, |a, b| a + b)
}

fn read_schematic(reader: &mut impl Read) -> EngineSchema {
    BufReader::new(reader)
        .lines()
        .map(Result::ok)
        .filter_map(|x| x)
        .fold(EngineSchema::new(), |mut a, b| {
            a.add_line(b);
            a
        })
}

struct EngineSchema {
    width: usize,
    height: usize,
    data: Vec<char>,
}

impl EngineSchema {
    fn new() -> Self {
        Self {
            width: 0,
            height: 0,
            data: vec![],
        }
    }

    fn add_line(&mut self, line: String) {
        let chars = line.trim().chars();
        let before = self.data.len();
        self.data.extend(chars);
        let after = self.data.len();
        let width = after - before;
        if self.width == 0 {
            self.width = width;
        } else if width != self.width {
            todo!("Handle incorrect line width properly");
        }
        self.height += 1;
    }

    fn get_number(&self, position: Position) -> Option<Number> {
        let Some(c) = self.get_char(position) else {
            return None;
        };
        if !c.is_digit(10) {
            return None;
        }
        let start = {
            let mut pos = position.clone();
            loop {
                let Some(new_pos) = pos.left() else {
                    break pos;
                };
                let Some(c) = self.get_char(new_pos) else {
                    break pos;
                };
                if !c.is_digit(10) {
                    break pos;
                }
                pos = new_pos;
            }
        };
        let end = {
            let mut pos = position.clone();
            loop {
                let new_pos = pos.right();
                let Some(c) = self.get_char(new_pos) else {
                    break pos;
                };
                if !c.is_digit(10) {
                    break pos;
                }
                pos = new_pos;
            }
        };
        let start_idx = self.get_vec_idx(start).unwrap();
        let end = self.get_vec_idx(end).unwrap();
        let str: String = self.data[start_idx..=end].iter().collect();
        Some(Number {
            number: str.parse().ok()?,
            start_pos: start,
        })
    }


    fn get_vec_idx(&self, position: Position) -> Option<usize> {
        let Position { x, y } = position;
        if x >= self.width || y >= self.height {
            None
        } else {
            Some(y * self.width + x)
        }
    }

    fn get_char(&self, position: Position) -> Option<char> {
        let idx = self.get_vec_idx(position)?;
        Some(self.data[idx])
    }

    fn pos(&self, idx: usize) -> Position {
        Position {
            x: idx % self.height,
            y: idx / self.height,
        }
    }

    fn iter_parts(&self) -> EnginePartIterator {
        EnginePartIterator {
            schema: self,
            vector_iter: self.data.iter(),
            idx: 0,
        }
    }
}

struct EnginePartIterator<'engine_schema> {
    vector_iter: std::slice::Iter<'engine_schema, char>,
    idx: usize,
    schema: &'engine_schema EngineSchema,
}

impl<'engine_schema> Iterator for EnginePartIterator<'engine_schema> {
    type Item = EnginePart;

    fn next(&mut self) -> Option<Self::Item> {
        fn get_part_no(part_pos: Position, schema: &EngineSchema) -> HashSet<Number> {
            let mut numbers = HashSet::new();
            let bottom = part_pos.down();
            if let Some(part_no) = schema.get_number(bottom) {
                numbers.insert(part_no);
            }
            let bottom_right = bottom.right();
            if let Some(part_no) = schema.get_number(bottom_right) {
                numbers.insert(part_no);
            }
            let right = part_pos.right();
            if let Some(part_no) = schema.get_number(right) {
                numbers.insert(part_no);
            }
            if let Some(up) = part_pos.up() {
                if let Some(part_no) = schema.get_number(up) {
                    numbers.insert(part_no);
                }
                let up_right = up.right();
                if let Some(part_no) = schema.get_number(up_right) {
                    numbers.insert(part_no);
                }
                if let Some(up_left) = up.left() {
                    if let Some(part_no) = schema.get_number(up_left) {
                        numbers.insert(part_no);
                    }
                    let left = up_left.down();
                    if let Some(part_no) = schema.get_number(left) {
                        numbers.insert(part_no);
                    }
                    let bottom_left = left.down();
                    if let Some(part_no) = schema.get_number(bottom_left) {
                        numbers.insert(part_no);
                    }
                }
            } else {
                if let Some(left) = part_pos.left() {
                    if let Some(part_no) = schema.get_number(left) {
                        numbers.insert(part_no);
                    }
                    let bottom_left = left.down();
                    if let Some(part_no) = schema.get_number(bottom_left) {
                        numbers.insert(part_no);
                    }
                }
            }
            numbers
        }
        loop {
            let symbol = loop {
                let c = self.vector_iter.next()?;
                self.idx += 1;
                if !(*c == '.' || c.is_digit(10)) {
                    break *c;
                }
            };
            let pos = self.schema.pos(self.idx - 1);
            let part_no = get_part_no(pos, self.schema);
            return Some(EnginePart {
                part_no,
                pos,
                symbol,
            });
        }
    }
}

struct EnginePart {
    symbol: char,
    pos: Position,
    part_no: HashSet<Number>,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Number {
    number: u32,
    start_pos: Position,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn right(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y,
        }
    }

    fn left(&self) -> Option<Self> {
        Some(Self {
            x: self.x.checked_sub(1)?,
            y: self.y,
        })
    }

    fn up(&self) -> Option<Self> {
        Some(Self {
            x: self.x,
            y: self.y.checked_sub(1)?,
        })
    }

    fn down(&self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1,
        }
    }
}
