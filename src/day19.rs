use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};
pub fn tokenize(moles: String) -> Vec<String> {
    let mut res = vec![];
    let mut buf = String::new();

    for c in moles.chars() {
        if c.is_uppercase() {
            if !buf.is_empty() {
                res.push(buf.clone());
                buf.clear();
            }
        }
        buf.push(c);
    }
    if !buf.is_empty() {
        res.push(buf);
    }
    res
}
pub fn day19() {
    let fs = read_to_string("puzzles/puz19.txt").unwrap();
    let k: Vec<&str> = fs.lines().collect();
    let (formulas, molecule) = k.split_at(k.len() - 1);
    let mut formulas: Vec<Vec<&str>> = formulas
        .iter()
        .map(|x| x.split_whitespace().collect())
        .collect();
    formulas.retain(|x| !x.is_empty());
    let mut hs: HashMap<String, Vec<String>> = HashMap::new();
    for i in formulas {
        hs.entry(i[0].to_string())
            .or_insert(vec![])
            .push(i[2].to_string());
    }
    let moles = molecule[0];
    let res = tokenize(moles.to_string());
    let mut hk: HashSet<String> = HashSet::new();
    for pat in &res {
        for (start, _) in moles.match_indices(pat.as_str()) {
            let end = start + pat.len();
            if let Some(vals) = hs.get(pat) {
                for v in vals {
                    let mut new_s = String::new();
                    new_s.push_str(&moles[..start]);
                    new_s.push_str(v);
                    new_s.push_str(&moles[end..]);
                    hk.insert(new_s);
                }
            }
        }
    }
    let elements = moles.bytes().filter(u8::is_ascii_uppercase).count();
    let rn = moles.matches("Rn").count();
    let ar = moles.matches("Ar").count();
    let y = moles.matches('Y').count();
    let res = elements - rn - ar - 2 * y - 1;
    println!("{res}");
}
