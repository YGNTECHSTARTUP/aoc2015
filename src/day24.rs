use iter_tools::Itertools;
use std::fs::read_to_string;

pub fn day24() {
    let k: Vec<u32> = read_to_string("puzzles/puz24.txt")
        .unwrap()
        .lines()
        .map(|x| x.parse().unwrap())
        .collect();

    let total: u32 = k.iter().copied().sum();
    let target = total / 4;
    for size1 in 1..k.len() {
        for g1 in k.iter().combinations(size1) {
            if g1.iter().copied().sum::<u32>() != target {
                continue;
            }

            let rest1: Vec<&u32> = k.iter().filter(|x| !g1.contains(x)).collect();

            for size2 in 1..rest1.len() {
                for g2 in rest1.iter().combinations(size2) {
                    if g2.iter().map(|&&x| x).sum::<u32>() != target {
                        continue;
                    }

                    let rest2: Vec<&u32> =
                        rest1.iter().filter(|x| !g2.contains(x)).copied().collect();

                    for size3 in 1..rest2.len() {
                        for g3 in rest2.iter().combinations(size3) {
                            if g3.iter().map(|&&x| x).sum::<u32>() != target {
                                continue;
                            }

                            let rest3: Vec<&u32> =
                                rest2.iter().filter(|x| !g3.contains(x)).copied().collect();

                            if rest3.iter().copied().sum::<u32>() == target {
                                println!("G1 = {:?}", g1);
                                println!("G2 = {:?}", g2);
                                println!("G3 = {:?}", g3);
                                println!("G4 = {:?}", rest3);

                                let p: u128 = g1.iter().map(|&&x| x as u128).product();
                                println!("QE = {p}");
                                return;
                            }
                        }
                    }
                }
            }
        }
    }
}
