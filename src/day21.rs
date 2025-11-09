use iter_tools::dependency::itertools::iproduct;

pub struct Character {
    damage: i32,
    armor: i32,
    hit_points: i32,
    cost: i32,
}
pub struct Item {
    name: String,
    cost: i32,
    dmg: i32,
    arm: i32,
}

impl Character {
    pub fn new(w: &Item, a: &Item, r1: &Item, r2: &Item, hp: i32) -> Self {
        Self {
            damage: w.dmg + a.dmg + r1.dmg + r2.dmg,
            armor: a.arm + w.arm + r1.arm + r2.arm,
            hit_points: hp,
            cost: w.cost + a.cost + r1.cost + r2.cost,
        }
    }
    pub fn defeats(&self, boss: &Character) -> bool {
        let my_turn_to_win = div_ceil(boss.hit_points, calc_dmg(self.damage, boss.armor));
        let boss_turn_to_win = div_ceil(self.hit_points, calc_dmg(boss.damage, self.armor));
        my_turn_to_win <= boss_turn_to_win
    }
}

fn calc_dmg(damage: i32, armor: i32) -> i32 {
    if damage > armor { damage - armor } else { 1 }
}
fn div_ceil(a: i32, b: i32) -> i32 {
    (a / b) + i32::from(a % b != 0)
}

pub fn day21() {
    let weapons = vec![
        Item {
            name: "Dagger".to_string(),
            cost: 8,
            dmg: 4,
            arm: 0,
        },
        Item {
            name: "Shortsword".to_string(),
            cost: 10,
            dmg: 5,
            arm: 0,
        },
        Item {
            name: "Warhammer".to_string(),
            cost: 25,
            dmg: 6,
            arm: 0,
        },
        Item {
            name: "Longsword".to_string(),
            cost: 40,
            dmg: 7,
            arm: 0,
        },
        Item {
            name: "Greataxe".to_string(),
            cost: 74,
            dmg: 8,
            arm: 0,
        },
    ];

    let armors = vec![
        Item {
            name: "Leather".to_string(),
            cost: 13,
            dmg: 0,
            arm: 1,
        },
        Item {
            name: "Chainmail".to_string(),
            cost: 31,
            dmg: 0,
            arm: 2,
        },
        Item {
            name: "Splintmail".to_string(),
            cost: 53,
            dmg: 0,
            arm: 3,
        },
        Item {
            name: "Bandedmail".to_string(),
            cost: 75,
            dmg: 0,
            arm: 4,
        },
        Item {
            name: "Platemail".to_string(),
            cost: 102,
            dmg: 0,
            arm: 5,
        },
        Item {
            name: "none".to_string(),
            cost: 0,
            dmg: 0,
            arm: 0,
        },
    ];
    let rings = vec![
        Item {
            name: "Damage +1".to_string(),
            cost: 25,
            dmg: 1,
            arm: 0,
        },
        Item {
            name: "Damage +2".to_string(),
            cost: 50,
            dmg: 2,
            arm: 0,
        },
        Item {
            name: "Damage +3".to_string(),
            cost: 100,
            dmg: 3,
            arm: 0,
        },
        Item {
            name: "Defense +1".to_string(),
            cost: 20,
            dmg: 0,
            arm: 1,
        },
        Item {
            name: "Defense +2".to_string(),
            cost: 40,
            dmg: 0,
            arm: 2,
        },
        Item {
            name: "Defense +3".to_string(),
            cost: 80,
            dmg: 0,
            arm: 3,
        },
        Item {
            name: "none".to_string(),
            cost: 0,
            dmg: 0,
            arm: 0,
        },
    ];
    let boss = Character {
        damage: 8,
        armor: 1,
        hit_points: 104,
        cost: 0,
    };
    let k = iproduct!(&armors, &weapons, &rings, &rings)
        .filter(|(_, _, r1, r2)| {
            r1.name != r2.name || r1.name == "none".to_string() || r2.name == "none".to_string()
        })
        .map(|(a, w, r1, r2)| Character::new(&w, &a, &r1, &r2, 100))
        .filter(|l| !l.defeats(&boss))
        .max_by(|a, b| a.cost.cmp(&b.cost))
        .unwrap()
        .cost;
    println!("{:?}", k);
}
