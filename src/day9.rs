use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

use iter_tools::Itertools;

pub fn day9() {
    //     let binding = "Tristram to AlphaCentauri = 34
    // Tristram to Snowdin = 100
    // Tristram to Tambi = 63
    // Tristram to Faerun = 108
    // Tristram to Norrath = 111
    // Tristram to Straylight = 89
    // Tristram to Arbre = 132
    // AlphaCentauri to Snowdin = 4
    // AlphaCentauri to Tambi = 79
    // AlphaCentauri to Faerun = 44
    // AlphaCentauri to Norrath = 147
    // AlphaCentauri to Straylight = 133
    // AlphaCentauri to Arbre = 74
    // Snowdin to Tambi = 105
    // Snowdin to Faerun = 95
    // Snowdin to Norrath = 48
    // Snowdin to Straylight = 88
    // Snowdin to Arbre = 7
    // Tambi to Faerun = 68
    // Tambi to Norrath = 134
    // Tambi to Straylight = 107
    // Tambi to Arbre = 40
    // Faerun to Norrath = 11
    // Faerun to Straylight = 66
    // Faerun to Arbre = 144
    // Norrath to Straylight = 115
    // Norrath to Arbre = 135
    // Straylight to Arbre = 127";
    let binding = read_to_string("puzzles/puz9.txt").unwrap();
    let fs: Vec<Vec<&str>> = binding
        .lines()
        .into_iter()
        .map(|x| x.split(" ").collect())
        .collect();
    let mut hs: HashMap<(String, String), u32> = HashMap::new();
    let mut cities: HashSet<String> = HashSet::new();
    for i in fs.iter() {
        hs.insert((i[0].to_string(), i[2].to_string()), i[4].parse().unwrap());
        hs.insert((i[2].to_string(), i[0].to_string()), i[4].parse().unwrap());
        cities.insert(i[0].to_string());
        cities.insert(i[2].to_string());
    }
    let mut max_distance = u32::MIN;
    for route in cities.iter().permutations(cities.len()) {
        let mut route = route.iter();
        let mut c1 = route.next().unwrap();
        let mut dist = 0;

        while let Some(c2) = route.next() {
            dist += *hs.get(&(c1.to_string(), c2.to_string())).unwrap();
            c1 = c2
        }
        max_distance = max_distance.max(dist);
    }
    println!("{:?}", max_distance);
}
