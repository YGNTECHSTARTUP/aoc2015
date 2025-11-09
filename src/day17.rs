use iter_tools::Itertools;
use std::fs::read_to_string;
pub fn day17() {
    let k: Vec<u32> = read_to_string("src/puzzles/puz17.txt")
        .unwrap()
        .lines()
        .map(|k| k.parse().unwrap())
        .collect();
    let mut total_comb = 0;
    let mut mc: Vec<u32> = vec![0; k.len()];
    for i in 1..k.len() {
        total_comb += k
            .iter()
            .combinations(i)
            .filter(|pk| pk.into_iter().cloned().sum::<u32>() == 150)
            .count();
        mc[i] = total_comb as u32;
    }
    println!("{:?}", mc);
    println!("{:?}", total_comb);
}
