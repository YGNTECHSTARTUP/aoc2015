use iter_tools::Itertools;
use std::collections::{HashMap, HashSet};

pub fn day13() {
    let fs = std::fs::read_to_string("puzzles/puz13.txt").unwrap();

    let mut hmap: HashMap<(String, String), i32> = HashMap::new();
    let mut members: HashSet<String> = HashSet::new();
    let mut best = i32::MIN;
    let st: Vec<Vec<&str>> = fs.lines().map(|x| x.split_whitespace().collect()).collect();
    for i in st {
        let mut x: i32 = i[3].parse().unwrap();
        if i[2] == "gain" {
            x = x;
        } else {
            x = -x;
        }
        let mut k = i[10].to_string();
        if k.ends_with(".") {
            k.pop();
        }
        hmap.insert((i[0].to_string(), k.clone()), x);
        members.insert(i[0].to_string());
        members.insert(k);
    }
    for i in members.iter() {
        hmap.insert(("YGN".to_string(), i.to_string()), 0);
        hmap.insert((i.to_string(), "YGN".to_string()), 0);
    }
    members.insert("YGN".to_string());
    for i in members.iter().permutations(members.len()) {
        let mut total = 0;
        for k in i.windows(2) {
            total += hmap[&(k[0].to_string(), k[1].to_string())];
            total += hmap[&((k[1].to_string(), k[0].to_string()))];
        }
        total += hmap
            .get(&(i[0].to_string(), i[i.len() - 1].to_string()))
            .unwrap();
        total += hmap
            .get(&(i[i.len() - 1].to_string(), i[0].to_string()))
            .unwrap();
        best = best.max(total);
    }
    println!("{:?}", best);
}
