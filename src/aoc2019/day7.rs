use super::intcode;
use itertools::Itertools;
use rstest::rstest;
use std::str;

fn part_one(input: String) -> i64 {
    let mut max_power: i64 = 0;
    let base: Vec<i64> = vec![0, 1, 2, 3, 4];
    let instr: Vec<i64> = input
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    for perm in base.iter().permutations(base.len()).unique() {
        let mut power: i64 = 0;
        for phase in perm {
            let mut stdin: Vec<i64> = Vec::new();
            let mut stdout = Vec::new();
            let i = instr.clone();
            stdin.push(*phase);
            stdin.push(power);
            let mut vm = intcode::VirtualMachine::new(i);
            vm.pause_mode();
            vm.run(&mut stdin, &mut stdout);
            let mut s: String = str::from_utf8(&stdout).unwrap().to_string();
            s.truncate(s.len() - 1);

            power = s.parse::<i64>().unwrap();
        }
        if power > max_power {
            max_power = power;
        }
    }
    return max_power;
}

// this question was really badly worded or I was too tired to understand.. TODO(): clean me up.
fn part_two(input: String) -> i64 {
    let mut max_power: i64 = 0;
    let base: Vec<i64> = vec![5, 6, 7, 8, 9];
    let instr: Vec<i64> = input
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    for perm in base.iter().permutations(base.len()).unique() {
        let mut amps: Vec<intcode::VirtualMachine> = Vec::new();
        for i in 0..5 {
            amps.push(intcode::VirtualMachine::new(instr.clone()));
            amps[i].pause_mode();
        }
        let mut power: i64 = 0;
        let mut halting = false;
        let mut inp: Vec<i64> = vec![0];
        let mut first = true;
        while !halting {
            for (i, phase) in perm.iter().enumerate() {
                let mut stdin: Vec<i64> = Vec::new();
                let mut stdout = Vec::new();
                if first {
                    stdin.push(**phase);
                }
                stdin.append(&mut inp);
                let (status, _val) = amps[i].run(&mut stdin, &mut stdout);
                if status == intcode::Status::Halted {
                    halting = true;
                } else {
                    inp = str::from_utf8(&stdout)
                        .unwrap()
                        .to_string()
                        .lines()
                        .filter(|&x| !x.is_empty())
                        .map(|x| x.parse::<i64>().unwrap())
                        .collect();
                    if i == 4 {
                        power = inp[0];
                    }
                }
            }
            first = false;
        }
        if power > max_power {
            max_power = power;
        }
    }
    return max_power;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rstest]
    #[case("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0", 43210)]
    #[case("3,8,1001,8,10,8,105,1,0,0,21,42,51,76,93,110,191,272,353,434,99999,3,9,1002,9,2,9,1001,9,3,9,1002,9,3,9,1001,9,2,9,4,9,99,3,9,1002,9,3,9,4,9,99,3,9,1002,9,4,9,101,5,9,9,1002,9,3,9,1001,9,4,9,1002,9,5,9,4,9,99,3,9,1002,9,5,9,101,3,9,9,102,5,9,9,4,9,99,3,9,1002,9,5,9,101,5,9,9,1002,9,2,9,4,9,99,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,1,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,101,1,9,9,4,9,99,3,9,1001,9,1,9,4,9,3,9,1001,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,99,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,101,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,99,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,1001,9,1,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,1,9,4,9,99,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,102,2,9,9,4,9,99", 567045)]

    fn partone_test(#[case] input: String, #[case] expected1: i64) {
        let first = part_one(input);
        assert_eq!(expected1, first);
    }

    #[rstest]
    #[case(
        "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5",
        139629729
    )]
    #[case("3,8,1001,8,10,8,105,1,0,0,21,42,51,76,93,110,191,272,353,434,99999,3,9,1002,9,2,9,1001,9,3,9,1002,9,3,9,1001,9,2,9,4,9,99,3,9,1002,9,3,9,4,9,99,3,9,1002,9,4,9,101,5,9,9,1002,9,3,9,1001,9,4,9,1002,9,5,9,4,9,99,3,9,1002,9,5,9,101,3,9,9,102,5,9,9,4,9,99,3,9,1002,9,5,9,101,5,9,9,1002,9,2,9,4,9,99,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,1,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,101,1,9,9,4,9,99,3,9,1001,9,1,9,4,9,3,9,1001,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,99,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,101,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,99,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,1001,9,1,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,1,9,4,9,99,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,102,2,9,9,4,9,99", 39016654)]

    fn parttwo_test(#[case] input: String, #[case] expected1: i64) {
        let first = part_two(input);
        assert_eq!(expected1, first);
    }
}
