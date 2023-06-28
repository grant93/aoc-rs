use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_lines_to_i64s<P>(filename: P) -> Vec<i64>
where
    P: AsRef<Path>,
{
    let mut ints: Vec<i64> = Vec::new();
    let lines = read_lines(filename).unwrap();
    for line in lines {
        ints.push(line.unwrap().parse::<i64>().unwrap())
    }
    ints
}
