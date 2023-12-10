use std::io::{BufRead, BufReader, Read};
use std::ops::Sub;
use std::str::FromStr;
use itertools::*;

pub(crate) fn a(reader: &mut impl Read) -> i32 {
    BufReader::new(reader).lines().flatten()
        .filter_map(|x| {
            let items: Vec<i32> = x.split(' ')
                .filter_map(|x| i32::from_str(&x).ok())
                .collect();
            let next = extrapolate_next(items);
            next
        })
        .fold(0, |a, b| a + b)
}

fn extrapolate_next(data: Vec<i32>) -> Option<i32> {
    let mut buf = Vec::with_capacity(data.len() - 1);
    let mut stack = Vec::with_capacity(data.len());
    stack.push(*data.last()?);
    buf.extend(Derivator::new(data.iter()));
    while !buf.iter().all(|&x| x == 0) {
        stack.push(*buf.last()?);
        let mut new_buf = Vec::with_capacity(buf.len() - 1);
        let derivator = Derivator::new(buf.into_iter());
        new_buf.extend(derivator);
        buf = new_buf;
    }
    for i in (1..stack.len()).rev() {
        stack[i - 1] = stack[i - 1] + stack[i];
    }

    Some(stack[0])
}

struct Derivator<I: Iterator> {
    inner: TupleWindows<I, (I::Item, I::Item)>,
}


impl<I> Derivator<I> where I: Iterator, I::Item: Clone {
    fn new(seq: I) -> Self {
        Self { inner: seq.tuple_windows() }
    }
}

impl<I> Iterator for Derivator<I>
    where I: Iterator,
          I::Item: Sub + Clone,
{
    type Item = <<I as Iterator>::Item as Sub>::Output;

    fn next(&mut self) -> Option<Self::Item> {
        let (a, b) = self.inner.next()?;
        Some(b - a)
    }
}