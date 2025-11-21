use core::panic;
use std::{collections::HashMap, fs::read_to_string};

#[derive(Debug, Clone)]
struct Gate {
    in1: String,
    in2: String,
    op: String,
}

impl Gate {
    pub fn new(in1: String, in2: String, op: String) -> Gate {
        Gate { in1, in2, op }
    }
}

struct Challenge {
    hash: HashMap<String, Gate>,
    values: HashMap<String, u16>,
}

impl Challenge {
    pub fn new() -> Challenge {
        Challenge {
            hash: HashMap::new(),
            values: HashMap::new(),
        }
    }

    pub fn get_value(&mut self, node: String) -> u16 {
        // Literal number
        if node.chars().all(|x| x.is_ascii_digit()) {
            return node.parse::<u16>().unwrap();
        }

        // Cached value
        if let Some(&v) = self.values.get(&node) {
            return v;
        }

        // Get gate for this wire
        let g = self.hash.get(node.as_str()).unwrap().clone();

        // First input
        let v1: u16 = self.get_value(g.in1.clone());
        let op: String = g.op.clone();

        let result = match op.as_str() {
            "NONE" => v1, // direct wire / value
            "NOT" => !v1, // unary NOT
            _ => {
                // Binary ops: need second input
                let v2: u16 = self.get_value(g.in2.clone());
                match op.as_str() {
                    "AND" => v1 & v2,
                    "OR" => v1 | v2,
                    "LSHIFT" => v1 << v2,
                    "RSHIFT" => v1 >> v2,
                    _ => panic!("Unhandled Operation: {}", op),
                }
            }
        };

        // Memoize and return
        self.values.insert(node, result);
        result
    }

    pub fn getpart2(&mut self, b: u16) {
        self.hash.entry("b".to_string()).and_modify(|x| {
            x.in1 = b.to_string();
            x.in2 = "".to_string();
            x.op = "NONE".to_string();
        });
        self.values = HashMap::new();
    }
}

pub fn day7() {
    let k = read_to_string("puzzles/puz7.txt").unwrap();
    let mut chal = Challenge::new();

    for line in k.lines() {
        let (input, output) = line.split_once("->").unwrap();
        let input = input.trim();
        let output = output.trim();

        let parts: Vec<&str> = input.split_whitespace().collect();

        let gt: Gate = if parts[0] == "NOT" {
            // "NOT x -> h"
            Gate::new(parts[1].to_string(), "NONE".to_string(), "NOT".to_string())
        } else if parts.len() == 1 {
            // "123 -> x" or "lx -> a"
            Gate::new(parts[0].to_string(), "NONE".to_string(), "NONE".to_string())
        } else {
            // "x AND y -> d", "x LSHIFT 2 -> f", etc.
            Gate::new(
                parts[0].to_string(), // in1
                parts[2].to_string(), // in2
                parts[1].to_string(), // op: AND/OR/LSHIFT/RSHIFT
            )
        };

        chal.hash.insert(output.to_string(), gt);
    }

    println!("{:?}", chal.getpart2(46065));
    println!("{:?}", chal.get_value("a".to_string()));
}
