#![allow(dead_code)]
use crate::aoc2019::intcode;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::str;

fn convert_graphics(val: i64) -> char {
    match val {
        0 => ' ',
        1 => '|',
        2 => '+',
        3 => '-',
        4 => 'o',
        _ => panic!("fuk"),
    }
}

fn render_graphics(map: &mut HashMap<(i64, i64), char>, data: &[i64]) -> Option<i64> {
    match data[0] {
        -1 => Some(data[2]),
        _ => {
            map.insert((data[0], data[1]), convert_graphics(data[2]));
            None
        }
    }
}

fn print_map(map: &HashMap<(i64, i64), char>) {
    for y in 0..20 {
        let mut line = String::new();
        for x in 0..50 {
            line.push(*map.get(&(x, y)).unwrap_or(&' '));
        }
        println!("{}", line);
    }
}

fn find_ball(map: &HashMap<(i64, i64), char>) -> i64 {
    for (k, v) in map {
        if *v == 'o' {
            return k.0;
        }
    }
    0
}

fn find_paddle(map: &HashMap<(i64, i64), char>) -> i64 {
    for (k, v) in map {
        if *v == '-' {
            return k.0;
        }
    }
    0
}

fn run(instr: Vec<i64>, input: &mut Vec<i64>, stdout: &mut Vec<u8>) -> (usize, i64) {
    let mut vm = intcode::VirtualMachine::new(instr);
    let mut map: HashMap<(i64, i64), char> = HashMap::new();
    vm.pause_mode();
    let mut status = intcode::Status::Paused;
    let mut score: i64 = 0;
    let mut p1: usize = 0;
    loop {
        for _ in 0..3 {
            (status, _) = vm.run(input, stdout);
        }
        if status == intcode::Status::Halted {
            break;
        } else if status == intcode::Status::InputRequired {
            if p1 == 0 {
                p1 = map.iter().filter(|(_, val)| **val == '+').count()
            }
            let diff = find_paddle(&map) - find_ball(&map);
            match diff.cmp(&0) {
                Ordering::Greater => input.push(-1),
                Ordering::Less => input.push(1),
                Ordering::Equal => input.push(0),
            }
        } else if let Some(x) = render_graphics(
            &mut map,
            &str::from_utf8(stdout)
                .unwrap()
                .lines()
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<i64>>(),
        ) {
            score = x;
            if p1 > 0 && map.iter().filter(|(_, b)| **b == '+').count() == 0 {
                break;
            }
        }
        stdout.clear();
        //print_map(&map);
    }
    (p1, score)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![2,380,379,385,1008,2399,462045,381,1005,381,12,99,109,2400,1102,0,1,383,1101,0,0,382,20101,0,382,1,21001,383,0,2,21102,37,1,0,1105,1,578,4,382,4,383,204,1,1001,382,1,382,1007,382,44,381,1005,381,22,1001,383,1,383,1007,383,20,381,1005,381,18,1006,385,69,99,104,-1,104,0,4,386,3,384,1007,384,0,381,1005,381,94,107,0,384,381,1005,381,108,1105,1,161,107,1,392,381,1006,381,161,1102,-1,1,384,1106,0,119,1007,392,42,381,1006,381,161,1101,0,1,384,21002,392,1,1,21101,0,18,2,21102,1,0,3,21101,138,0,0,1105,1,549,1,392,384,392,21002,392,1,1,21102,1,18,2,21102,3,1,3,21101,0,161,0,1106,0,549,1101,0,0,384,20001,388,390,1,20101,0,389,2,21102,1,180,0,1106,0,578,1206,1,213,1208,1,2,381,1006,381,205,20001,388,390,1,21002,389,1,2,21102,1,205,0,1105,1,393,1002,390,-1,390,1102,1,1,384,20101,0,388,1,20001,389,391,2,21102,228,1,0,1105,1,578,1206,1,261,1208,1,2,381,1006,381,253,21002,388,1,1,20001,389,391,2,21102,253,1,0,1105,1,393,1002,391,-1,391,1101,1,0,384,1005,384,161,20001,388,390,1,20001,389,391,2,21101,279,0,0,1106,0,578,1206,1,316,1208,1,2,381,1006,381,304,20001,388,390,1,20001,389,391,2,21102,304,1,0,1106,0,393,1002,390,-1,390,1002,391,-1,391,1102,1,1,384,1005,384,161,21002,388,1,1,21002,389,1,2,21101,0,0,3,21101,0,338,0,1105,1,549,1,388,390,388,1,389,391,389,20101,0,388,1,20101,0,389,2,21102,1,4,3,21102,1,365,0,1106,0,549,1007,389,19,381,1005,381,75,104,-1,104,0,104,0,99,0,1,0,0,0,0,0,0,318,20,15,1,1,22,109,3,22101,0,-2,1,21202,-1,1,2,21102,1,0,3,21102,1,414,0,1106,0,549,21201,-2,0,1,22102,1,-1,2,21101,429,0,0,1106,0,601,2101,0,1,435,1,386,0,386,104,-1,104,0,4,386,1001,387,-1,387,1005,387,451,99,109,-3,2105,1,0,109,8,22202,-7,-6,-3,22201,-3,-5,-3,21202,-4,64,-2,2207,-3,-2,381,1005,381,492,21202,-2,-1,-1,22201,-3,-1,-3,2207,-3,-2,381,1006,381,481,21202,-4,8,-2,2207,-3,-2,381,1005,381,518,21202,-2,-1,-1,22201,-3,-1,-3,2207,-3,-2,381,1006,381,507,2207,-3,-4,381,1005,381,540,21202,-4,-1,-1,22201,-3,-1,-3,2207,-3,-4,381,1006,381,529,22101,0,-3,-7,109,-8,2106,0,0,109,4,1202,-2,44,566,201,-3,566,566,101,639,566,566,1201,-1,0,0,204,-3,204,-2,204,-1,109,-4,2106,0,0,109,3,1202,-1,44,593,201,-2,593,593,101,639,593,593,21002,0,1,-2,109,-3,2105,1,0,109,3,22102,20,-2,1,22201,1,-1,1,21102,443,1,2,21101,114,0,3,21102,1,880,4,21102,1,630,0,1106,0,456,21201,1,1519,-2,109,-3,2105,1,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,0,2,2,0,2,2,2,2,2,0,2,2,2,0,0,0,2,2,2,2,2,2,0,2,2,0,0,2,2,2,0,2,2,2,2,0,0,2,0,2,2,0,1,1,0,2,2,0,2,2,2,2,2,0,2,2,0,2,2,2,0,0,0,2,0,2,2,0,0,2,2,2,0,2,2,2,2,2,0,2,0,0,2,2,0,0,1,1,0,2,2,0,2,2,2,2,2,2,0,2,2,2,2,0,2,2,0,0,2,0,0,0,2,0,0,2,2,2,2,2,0,2,2,0,2,2,2,0,2,0,1,1,0,2,2,2,2,0,0,2,2,2,2,2,2,2,2,0,0,2,2,2,0,2,2,2,0,2,2,0,0,0,2,2,2,2,2,2,0,2,2,2,2,0,1,1,0,0,2,2,2,2,2,2,0,2,0,2,2,0,2,0,2,0,2,0,2,2,2,0,2,0,0,0,2,0,2,2,0,0,2,2,2,2,2,0,2,0,1,1,0,0,2,2,2,2,2,2,2,2,0,2,2,0,0,2,0,2,0,2,0,0,2,2,0,0,2,2,0,0,0,2,0,2,0,2,2,2,0,2,2,0,1,1,0,0,0,0,0,0,2,0,2,0,0,2,2,0,2,2,0,0,2,0,0,2,2,2,2,2,2,2,2,0,2,2,0,0,0,2,2,2,2,2,2,0,1,1,0,0,2,2,2,2,0,0,0,0,2,0,0,2,2,0,2,2,2,2,2,2,0,2,2,2,0,2,2,2,0,2,2,2,2,2,2,0,2,0,2,0,1,1,0,2,0,2,2,2,2,2,0,0,2,2,2,2,2,0,2,2,2,2,2,2,0,2,0,0,0,2,2,0,2,0,0,2,0,2,2,2,2,2,2,0,1,1,0,2,2,2,2,2,0,2,2,2,2,2,2,2,0,2,2,2,2,2,0,2,2,2,2,2,2,0,2,2,2,2,2,2,0,0,0,2,2,0,2,0,1,1,0,0,0,0,2,2,2,2,2,2,2,0,2,2,2,2,2,0,2,0,2,2,0,2,2,0,2,2,2,0,2,2,0,0,2,0,2,0,2,2,0,0,1,1,0,0,2,2,2,2,0,2,0,2,2,0,2,2,2,2,2,2,0,2,0,0,0,0,2,0,2,2,2,2,2,0,0,2,0,2,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,3,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,56,72,33,24,73,85,29,45,48,12,58,96,32,17,48,17,9,49,78,6,48,98,91,23,28,37,16,54,30,88,69,69,23,59,33,61,89,94,79,64,42,13,82,45,98,57,57,80,6,2,67,86,26,84,80,47,72,38,22,73,75,4,3,47,96,34,52,22,26,90,74,84,37,68,37,73,86,59,27,67,52,22,63,92,41,46,68,93,55,3,6,78,32,82,8,34,41,77,29,92,77,81,17,90,40,50,27,80,82,96,66,21,67,96,69,12,39,65,93,7,91,97,55,95,64,9,91,48,23,44,96,67,38,43,26,70,64,17,47,98,35,54,89,31,67,18,36,42,52,19,71,4,47,21,43,77,6,64,45,94,49,52,54,85,9,78,73,44,56,3,37,15,45,16,78,98,82,27,59,13,26,75,73,18,74,20,63,65,56,55,98,34,10,97,55,70,51,25,9,16,10,79,49,58,13,92,19,25,79,33,48,5,78,86,94,48,39,3,43,90,35,45,56,60,51,92,4,52,64,63,18,70,44,82,70,29,72,53,91,36,75,95,57,61,42,79,98,26,8,73,10,3,69,95,69,39,13,70,90,66,96,97,21,35,38,43,21,79,91,5,92,93,48,25,31,15,39,58,51,68,46,93,10,56,16,5,54,34,54,68,22,97,18,14,96,52,92,62,62,62,43,62,73,41,85,36,81,81,1,41,92,94,78,32,72,15,30,54,86,1,60,28,20,94,15,52,60,68,63,15,45,39,66,65,42,35,28,31,83,59,87,69,83,22,58,45,22,70,86,98,44,13,37,24,67,80,7,67,16,10,88,54,60,76,97,37,63,31,61,91,10,61,97,76,59,40,28,15,45,50,86,61,30,11,85,87,53,10,88,40,69,82,60,57,38,74,35,44,33,98,80,47,3,51,56,12,28,86,26,91,45,10,92,18,63,4,66,47,73,18,57,51,32,79,25,41,61,68,78,34,71,3,33,29,40,25,15,72,88,51,20,76,70,10,20,38,13,27,92,97,60,22,54,73,20,51,27,87,51,41,73,61,1,31,94,11,74,56,34,9,74,31,20,91,63,75,1,54,62,31,30,60,74,67,13,83,65,10,63,38,65,75,94,85,98,53,59,63,42,21,93,13,55,36,76,53,14,30,71,2,84,16,82,87,57,74,57,29,48,14,73,4,22,91,81,94,41,67,27,82,20,4,89,43,92,36,70,29,45,82,65,49,2,63,78,18,13,75,76,50,85,64,37,4,57,41,18,15,65,70,44,85,72,11,36,35,84,4,70,49,47,20,10,80,79,59,89,1,87,5,22,87,31,23,38,35,49,71,33,46,81,64,43,59,46,51,62,33,89,61,66,64,92,23,30,56,17,71,85,18,2,72,2,42,31,13,53,35,17,91,73,73,48,95,20,26,23,10,65,4,40,6,79,49,84,7,15,49,90,45,24,42,76,21,97,3,63,42,30,92,55,38,44,53,67,44,42,36,28,9,17,66,92,44,51,55,57,59,6,50,52,97,21,45,19,17,21,76,86,32,23,56,78,93,97,13,93,87,32,83,89,23,21,63,40,87,83,95,95,74,57,60,82,48,45,18,93,63,74,31,30,43,50,28,69,60,43,81,86,67,64,17,67,27,79,49,92,21,71,59,32,83,29,72,3,62,47,95,76,63,32,53,32,28,75,50,22,37,43,20,10,13,80,80,19,43,55,23,14,70,32,80,4,44,4,40,35,44,55,41,68,80,68,25,27,97,39,30,24,42,52,88,87,36,23,83,58,50,85,60,97,72,97,51,37,83,40,59,52,25,83,8,76,14,20,94,43,45,75,47,12,67,46,56,30,74,1,28,41,42,74,21,36,22,80,69,23,12,62,25,39,77,8,46,56,64,43,34,8,54,85,43,20,84,24,13,64,92,68,7,61,49,46,16,87,54,24,94,70,63,63,33,43,30,29,34,22,23,98,20,90,14,77,27,89,39,13,3,77,47,462045], 318, 16309)]
    fn run_test(#[case] instr: Vec<i64>, #[case] expected: usize, #[case] expected2: i64) {
        let mut stdout = Vec::new();
        let (p1, p2) = run(instr.clone(), &mut Vec::new(), &mut stdout);
        assert_eq!(expected, p1);
        assert_eq!(expected2, p2);
    }
}
