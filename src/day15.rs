use std::fs::read_to_string;

pub fn day15() {
    let fd = read_to_string("puz15.txt").unwrap();
    let mut max_score = 0;
    let mut stats: Vec<(i32, i32, i32, i32, i32)> = Vec::new();
    for i in fd.lines() {
        let k: Vec<String> = i.split(" ").map(|l| l.replace(",", "")).collect();
        stats.push((
            k[2].parse().unwrap(),
            k[4].parse().unwrap(),
            k[6].parse().unwrap(),
            k[8].parse().unwrap(),
            k[10].parse().unwrap(),
        ));
    }
    let mut veca: Vec<Vec<i32>> = vec![vec![]; 5];
    for (cap, dur, fla, tex, cal) in stats.iter() {
        veca[0].push(*cap);
        veca[1].push(*dur);
        veca[2].push(*fla);
        veca[3].push(*tex);
        veca[4].push(*cal);
    }

    for a in 0..101 {
        for b in 0..(101 - a) {
            for c in 0..(101 - b - a) {
                let d = 100 - a - b - c;
                let mut ak: Vec<i32> = vec![];
                // for i in veca.iter() {
                //     let k: i32 = i
                //         .iter()
                //         .zip(vec![a, b, c, d])
                //         .map(|x| {
                //             x.0 * x.1;
                //         })
                //         .sum();
                //     ak.push(max_score.max(k));
                // }
                for props in &veca {
                    let total: i32 = props
                        .iter()
                        .zip([a as i32, b as i32, c as i32, d as i32])
                        .map(|(p, amt)| p * amt)
                        .sum();
                    ak.push(total);
                }
                let cal = ak[4];
                if cal == 500 {
                    let t: i32 = ak[..4].iter().map(|x| x.max(&0)).product();
                    max_score = max_score.max(t);
                }
            }
        }
    }
    println!("{:?}", max_score);
}
