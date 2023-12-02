mod day1;
mod day2;

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