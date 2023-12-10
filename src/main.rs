mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day7;
mod day8;
mod day9;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args = std::env::args();
    let (base, n, p) = {
        let mut iter = args.skip(1).take(3);
        (iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap())
    };
    let path = format!("./{base}/input{n}.txt");
    // println!("{}", path);
    let mut file = std::fs::File::open(path)?;
    match (n.as_str(), p.as_str()) {
        ("1", "a") => println!("{}", day1::a(&mut file)),
        ("1", "b") => println!("{}", day1::b(&mut file)),
        ("2", "a") => println!("{}", day2::a(&mut file)),
        ("2", "b") => println!("{}", day2::b(&mut file)),
        ("3", "a") => println!("{}", day3::a(&mut file)),
        ("3", "b") => println!("{}", day3::b(&mut file)),
        ("4", "a") => println!("{}", day4::a(&mut file)),
        ("4", "b") => println!("{}", day4::b(&mut file)),
        ("5", "a") => println!("{}", day5::a(&mut file).expect("Something went wrong in computations")),
        ("5", "b") => println!("{}", day5::b(&mut file).expect("Something went wrong in computations")),
        ("7", "a") => println!("{}", day7::a(&mut file)),
        ("7", "b") => println!("{}", day7::b(&mut file)),
        ("8", "a") => println!("{}", day8::a(&mut file).expect("Something went wrong in computations")),
        ("8", "b") => println!("{}", day8::b(&mut file).expect("Something went wrong in computations")),
        ("9", "a") => println!("{}", day9::a(&mut file)),
        _ => println!("Not solved yet!")
    };
    Ok(())
}

trait Id {
    fn id(self) -> Self;
}

impl<T> Id for T {
    fn id(self) -> Self {
        self
    }
}