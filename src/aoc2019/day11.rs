#![allow(dead_code)]
use crate::aoc2019::intcode;
use std::collections::HashMap;
use std::str;

#[repr(u8)]
enum Direction {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

impl From<u8> for Direction {
    fn from(i: u8) -> Self {
        match i {
            0 => Direction::Up,
            1 => Direction::Right,
            2 => Direction::Down,
            3 => Direction::Left,
            _ => panic!("AHH"),
        }
    }
}

fn print_map(map: &HashMap<(i64, i64), u8>) {
    for y in (-5..1).rev() {
        let mut line = String::new();
        for x in -20..50 {
            line.push(if *map.get(&(x, y)).unwrap_or(&0) == 0u8 {
                '.'
            } else {
                '#'
            })
        }
        println!("{}", line);
    }
}

fn partone(instr: Vec<i64>, input: &mut Vec<i64>, stdout: &mut Vec<u8>, p2: bool) -> usize {
    let mut vm = intcode::VirtualMachine::new(instr);
    let mut map: HashMap<(i64, i64), u8> = HashMap::new();
    let mut coords: (i64, i64) = (0, 0);
    let mut dir: Direction = Direction::Up;
    vm.pause_mode();
    if p2 {
        map.insert((0, 0), 1);
    }
    let mut status = intcode::Status::Paused;
    while status != intcode::Status::Halted {
        input.push(*map.get(&coords).unwrap_or(&0) as i64);
        vm.run(input, stdout);
        let colour = str::from_utf8(stdout)
            .unwrap()
            .lines()
            .last()
            .unwrap()
            .parse::<u8>()
            .unwrap();
        (status, _) = vm.run(input, stdout);
        if str::from_utf8(stdout)
            .unwrap()
            .lines()
            .last()
            .unwrap()
            .parse::<u8>()
            .unwrap()
            == 0
        {
            let a = dir as u8;
            dir = if a.wrapping_sub(1) == u8::MAX {
                Direction::Left
            } else {
                Direction::from(a - 1)
            };
        } else {
            let a = dir as u8;
            dir = if a + 1 == 4 {
                Direction::Up
            } else {
                Direction::from(a + 1)
            };
        }
        map.insert(coords, colour);
        match dir {
            Direction::Up => coords = (coords.0, coords.1 + 1),
            Direction::Right => coords = (coords.0 + 1, coords.1),
            Direction::Down => coords = (coords.0, coords.1 - 1),
            Direction::Left => coords = (coords.0 - 1, coords.1),
        };
    }
    if p2 {
        print_map(&map);
    }
    map.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![3,8,1005,8,332,1106,0,11,0,0,0,104,1,104,0,3,8,102,-1,8,10,101,1,10,10,4,10,108,1,8,10,4,10,101,0,8,28,3,8,102,-1,8,10,1001,10,1,10,4,10,1008,8,1,10,4,10,101,0,8,51,1,1103,5,10,1,1104,9,10,2,1003,0,10,1,5,16,10,3,8,102,-1,8,10,101,1,10,10,4,10,108,0,8,10,4,10,1001,8,0,88,1006,0,2,1006,0,62,2,8,2,10,3,8,1002,8,-1,10,101,1,10,10,4,10,1008,8,1,10,4,10,102,1,8,121,1006,0,91,1006,0,22,1006,0,23,1006,0,1,3,8,102,-1,8,10,1001,10,1,10,4,10,1008,8,1,10,4,10,101,0,8,155,1006,0,97,1,1004,2,10,2,1003,6,10,3,8,1002,8,-1,10,101,1,10,10,4,10,108,0,8,10,4,10,1002,8,1,187,1,104,15,10,2,107,9,10,1006,0,37,1006,0,39,3,8,1002,8,-1,10,1001,10,1,10,4,10,108,0,8,10,4,10,102,1,8,223,2,2,17,10,1,1102,5,10,3,8,1002,8,-1,10,101,1,10,10,4,10,108,0,8,10,4,10,1001,8,0,253,3,8,102,-1,8,10,1001,10,1,10,4,10,1008,8,1,10,4,10,1002,8,1,276,1006,0,84,3,8,102,-1,8,10,101,1,10,10,4,10,1008,8,0,10,4,10,1001,8,0,301,2,1009,9,10,1006,0,10,2,102,15,10,101,1,9,9,1007,9,997,10,1005,10,15,99,109,654,104,0,104,1,21102,1,936995738516,1,21101,0,349,0,1105,1,453,21102,1,825595015976,1,21102,1,360,0,1105,1,453,3,10,104,0,104,1,3,10,104,0,104,0,3,10,104,0,104,1,3,10,104,0,104,1,3,10,104,0,104,0,3,10,104,0,104,1,21102,46375541763,1,1,21101,0,407,0,1105,1,453,21102,1,179339005019,1,21101,0,418,0,1106,0,453,3,10,104,0,104,0,3,10,104,0,104,0,21102,825012036372,1,1,21102,441,1,0,1105,1,453,21101,988648461076,0,1,21101,452,0,0,1105,1,453,99,109,2,22102,1,-1,1,21102,40,1,2,21102,484,1,3,21101,0,474,0,1106,0,517,109,-2,2105,1,0,0,1,0,0,1,109,2,3,10,204,-1,1001,479,480,495,4,0,1001,479,1,479,108,4,479,10,1006,10,511,1102,1,0,479,109,-2,2105,1,0,0,109,4,2102,1,-1,516,1207,-3,0,10,1006,10,534,21101,0,0,-3,21202,-3,1,1,22101,0,-2,2,21102,1,1,3,21102,553,1,0,1106,0,558,109,-4,2106,0,0,109,5,1207,-3,1,10,1006,10,581,2207,-4,-2,10,1006,10,581,22102,1,-4,-4,1105,1,649,21202,-4,1,1,21201,-3,-1,2,21202,-2,2,3,21101,0,600,0,1105,1,558,21201,1,0,-4,21101,0,1,-1,2207,-4,-2,10,1006,10,619,21101,0,0,-1,22202,-2,-1,-2,2107,0,-3,10,1006,10,641,22102,1,-1,1,21102,1,641,0,106,0,516,21202,-2,-1,-2,22201,-4,-2,-4,109,-5,2105,1,0], 2373, 250)]
    fn partone_test(#[case] instr: Vec<i64>, #[case] expected: usize, #[case] expected2: usize) {
        let mut stdout = Vec::new();
        let ans = partone(instr.clone(), &mut Vec::new(), &mut stdout, false);
        assert_eq!(expected, ans);
        let ans = partone(instr, &mut Vec::new(), &mut stdout, true);
        assert_eq!(expected2, ans);
    }
}
