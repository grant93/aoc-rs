#![allow(dead_code)]
use itertools::Itertools;
use regex::Regex;
use std::cmp::Ordering;
use std::iter::zip;

#[derive(Debug, Clone, Copy)]
struct Moon {
    pos: [i32; 3],
    vel: [i32; 3],
}

fn update_velocity(m1: &Moon, m2: &Moon) -> [i32; 3] {
    let mut newvel = [0i32; 3];
    for (i, vel) in newvel.iter_mut().enumerate() {
        *vel = match m1.pos[i].cmp(&m2.pos[i]) {
            Ordering::Greater => m1.vel[i] - 1,
            Ordering::Less => m1.vel[i] + 1,
            Ordering::Equal => m1.vel[i],
        };
    }
    newvel
}

fn lcm(a: usize, b: usize) -> usize {
    a * (b / gcd(a, b))
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn check_axis(results: &[Moon], first: &[Moon], pos: usize) -> bool {
    if results
        .iter()
        .zip(first.iter())
        .all(|(result, first)| result.pos[pos] == first.pos[pos])
    {
        let mut fp = false;
        for res in results {
            if res.vel[pos] != 0 {
                fp = true;
                break;
            }
        }
        if !fp {
            return true;
        }
    }
    false
}

fn run(input: String) -> (i32, usize) {
    let re = Regex::new(r"<x=(-?\d+), y=(-?\d+), z=(-?\d+)>").unwrap();
    let mut results: Vec<Moon> = vec![];

    for line in input.lines() {
        for (_, [x, y, z]) in re.captures_iter(line).map(|c| c.extract()) {
            results.push(Moon {
                pos: [
                    x.parse::<i32>().unwrap(),
                    y.parse::<i32>().unwrap(),
                    z.parse::<i32>().unwrap(),
                ],
                vel: [0i32, 0i32, 0i32],
            });
        }
    }

    let first = results.clone();
    let combinations: Vec<(usize, usize)> = (0..results.len()).tuple_combinations().collect();
    let mut count: usize = 0;
    let mut p1: i32 = 0;
    let mut steps: [usize; 3] = [0; 3];
    loop {
        count += 1;
        for (i1, i2) in &combinations {
            results.get_mut(*i1).unwrap().vel =
                update_velocity(results.get(*i1).unwrap(), results.get(*i2).unwrap());
            results.get_mut(*i2).unwrap().vel =
                update_velocity(results.get(*i2).unwrap(), results.get(*i1).unwrap());
        }
        for i in 0..4 {
            let moon1: &Moon = results.get(i).unwrap();
            results.get_mut(i).unwrap().pos = zip(moon1.pos, moon1.vel)
                .map(|(x, y)| x + y)
                .collect::<Vec<i32>>()
                .try_into()
                .unwrap();
        }

        if count == 1000 {
            p1 = results
                .iter()
                .map(|m| -> i32 {
                    m.pos.iter().map(|x| i32::abs(*x)).sum::<i32>()
                        * m.vel.iter().map(|x| i32::abs(*x)).sum::<i32>()
                })
                .sum()
        }

        for (i, item) in steps.iter_mut().enumerate() {
            if *item == 0 && check_axis(&results, &first, i) {
                *item = count;
            }
        }
        if steps.iter().all(|x| *x != 0) {
            break;
        }
    }
    (p1, lcm(steps[0], lcm(steps[1], steps[2])))
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use rstest::rstest;

    #[rstest]
    #[case(indoc! {"
    <x=-13, y=-13, z=-13>
<x=5, y=-8, z=3>
<x=-6, y=-10, z=-3>
<x=0, y=5, z=-5>"},

                    8044, 362375881472136)]

    fn partone_test(#[case] input: &str, #[case] expected1: i32, #[case] expected2: usize) {
        let (p1, p2) = run(input.to_string());
        assert_eq!(expected1, p1);
        assert_eq!(expected2, p2);
    }
}
