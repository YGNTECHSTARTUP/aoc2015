use std::{collections::HashMap, fs::read_to_string};

pub fn day16() {
    let fs = read_to_string("puzzles/puz16.txt").unwrap();

    let fk = "children: 3
cats: 7
samoyeds: 2
pomeranians: 3
akitas: 0
vizslas: 0
goldfish: 5
trees: 3
cars: 2
perfumes: 1";

    let mut vhs: HashMap<String, u32> = HashMap::new();
    for i in fk.lines() {
        let k: Vec<&str> = i.split(":").collect();
        println!("{:?}", k);
        vhs.insert(k[0].to_string(), k[1].trim().parse().unwrap());
    }
    for i in fs.lines() {
        let mut count: i32 = 0;
        let mut hs: HashMap<&String, u32> = HashMap::new();
        let ik: Vec<String> = i
            .split(" ")
            .map(|x| x.replace(",", ""))
            .map(|x| x.replace(":", ""))
            .collect();
        hs.insert(&ik[2], ik[3].parse().unwrap());
        hs.insert(&ik[4], ik[5].parse().unwrap());
        hs.insert(&ik[6], ik[7].parse().unwrap());
        for (&k, &v) in &hs {
            if *k == "cats".to_string() || *k == "trees".to_string() {
                if let Some(vk) = vhs.get(k) {
                    if *vk < v {
                        count += 1;
                        continue;
                    } else {
                        break;
                    }
                }
            }
            if *k == "pomeranians".to_string() || *k == "goldfish".to_string() {
                if let Some(vk) = vhs.get(k) {
                    if *vk > v {
                        count += 1;
                        continue;
                    } else {
                        break;
                    }
                }
            }
            if let Some(vk) = vhs.get(k) {
                if *vk == v {
                    count += 1;
                    continue;
                } else {
                    break;
                }
            };
        }
        if count == 3 {
            println!("{:?}", &ik[1]);
        }
    }
    // println!("{:?}", stats);
}
