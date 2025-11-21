use std::fs::read_to_string;

use regex::Regex;

fn num_to_coord(r: usize, c: usize) -> usize {
    let base = r + c - 1;
    let count = base * (base + 1) / 2;
    count - r + 1
} // gives the value present in the grid

fn coord_to_num(num: usize) -> usize {
    (1..num).fold(20151125, |acc, _| (acc * 252533) % 33554393)
}

pub fn day25() {
    let a = read_to_string("puzzles/puz25.txt").unwrap();
    let k = Regex::new(r"row (\d+), column (\d+)").unwrap();
    let p = k.captures(a.trim()).unwrap();
    let (r, c): (usize, usize) = (p[1].parse().unwrap(), p[2].parse().unwrap());
    println!("{}", coord_to_num(num_to_coord(r, c)))
}
