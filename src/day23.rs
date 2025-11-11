use std::fs::read_to_string;

pub fn day23() {
    let f = read_to_string("puzzles/puz23.txt").unwrap();
    let mut rega: u32 = 1;
    let mut regb: u32 = 0;
    enum Instructions {
        Half,
        Triple,
        Increment,
        Jump,
        JumpIfEven,
        JumpIfOne,
    }
    let mut ins: Vec<(usize, &str)> = f.lines().enumerate().collect();
    let mut pc = 0;
    while pc < ins.len() {
        if pc < 0 {
            pc = 0;
        }
        let (k, v) = (ins[pc].0, ins[pc].1);

        let words: Vec<&str> = v.split(" ").collect();
        let instruction = match words[0] {
            "hlf" => Instructions::Half,
            "tpl" => Instructions::Triple,
            "inc" => Instructions::Increment,
            "jmp" => Instructions::Jump,
            "jie" => Instructions::JumpIfEven,
            "jio" => Instructions::JumpIfOne,
            _ => panic!("err"),
        };

        match instruction {
            Instructions::Half => {
                if words[1] == "a" {
                    rega /= 2;
                    pc += 1;
                } else {
                    regb /= 2;
                    pc += 1;
                }
            }
            Instructions::Triple => {
                if words[1] == "a" {
                    rega *= 3;
                    pc += 1;
                } else {
                    regb *= 3;
                    pc += 1;
                }
            }
            Instructions::Increment => {
                if words[1] == "a" {
                    rega += 1;
                    pc += 1;
                } else {
                    regb += 1;
                    pc += 1;
                }
            }
            Instructions::Jump => {
                let (sign, num): (&str, usize) = (&words[1][..1], words[1][1..].parse().unwrap());
                if sign == "+" {
                    pc += num;
                } else {
                    pc -= num;
                }
            }

            Instructions::JumpIfEven => {
                let reg = words[1].trim_matches(',');
                let (sign, num): (&str, usize) = (&words[2][..1], words[2][1..].parse().unwrap());

                let val = if reg == "a" { rega } else { regb };

                if val % 2 == 0 {
                    if sign == "+" {
                        pc += num;
                    } else {
                        pc -= num;
                    }
                } else {
                    pc += 1;
                }
            }

            Instructions::JumpIfOne => {
                let reg = words[1].trim_matches(',');
                let (sign, num): (&str, usize) = (&words[2][..1], words[2][1..].parse().unwrap());

                let val = if reg == "a" { rega } else { regb };

                if val == 1 {
                    if sign == "+" {
                        pc += num;
                    } else {
                        pc -= num;
                    }
                } else {
                    pc += 1;
                }
            }
        }
    }
    println!("{regb}");
}
