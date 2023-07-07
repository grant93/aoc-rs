#![allow(dead_code)]

fn find_asteroids(map: &[Vec<char>]) -> Vec<(f64, f64)> {
    let mut coords: Vec<(f64, f64)> = Vec::new();
    for (y, row) in map.iter().enumerate() {
        for (x, val) in row.iter().enumerate() {
            if *val == '#' {
                coords.push((x as f64, y as f64));
            }
        }
    }
    coords
}

fn find_visible(pos: (f64, f64), coords: &Vec<(f64, f64)>) -> Vec<((f64, f64), f64, f64)> {
    let mut angles: Vec<((f64, f64), f64, f64)> = Vec::new();
    let (x1, y1) = pos;
    for (x2, y2) in coords {
        if x1 == *x2 && y1 == *y2 {
            continue;
        }
        let angle = (*x2 - x1).atan2(*y2 - y1);
        let distance = ((*x2 - x1).abs().powi(2) + (*y2 - y1).abs().powi(2)).sqrt();
        if !angles.iter().any(|&x| x.1 == angle) {
            angles.push(((*x2, *y2), angle, distance));
        } else {
            angles.iter_mut().for_each(|x| {
                if x.1 == angle && x.2 > distance {
                    *x = ((*x2, *y2), angle, distance);
                }
            });
        }
    }
    angles
}

fn find_optimal_asteroid(coords: &Vec<(f64, f64)>) -> (usize, (f64, f64)) {
    let mut max: usize = 0;
    let (mut x, mut y) = (0.0, 0.0);
    for (x1, y1) in coords {
        let angles = find_visible((*x1, *y1), coords);
        if angles.len() > max {
            max = angles.len();
            x = *x1;
            y = *y1;
        }
    }
    (max, (x, y))
}

fn run(input: String) -> (usize, usize) {
    let map: Vec<Vec<char>> = input
        .lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect();
    let coords = find_asteroids(&map);
    let (total, optimal_coords) = find_optimal_asteroid(&coords);
    println!("OPTIMAL: {:?}", optimal_coords);

    let mut map = find_visible(optimal_coords, &coords);
    map.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    let (coords, _, _) = map.get(map.len() - 200).unwrap();

    let part2 = (coords.0 as usize * 100) + coords.1 as usize;

    (total, part2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use rstest::rstest;

    #[rstest]
    #[case(indoc! {".#..##.###...#######
                    ##.############..##.
                    .#.######.########.#
                    .###.#######.####.#.
                    #####.##.#.##.###.##
                    ..#####..#.#########
                    ####################
                    #.####....###.#.#.##
                    ##.#################
                    #####.##.###..####..
                    ..######..##.#######
                    ####.##.####...##..#
                    .#####..#.######.###
                    ##...#.##########...
                    #.##########.#######
                    .####.#.###.###.#.##
                    ....##.##.###..#####
                    .#.#.###########.###
                    #.#.#.#####.####.###
                    ###.##.####.##.#..##"},
210, 802)]
    #[case(indoc! {".#.####..#.#...#...##..#.#.##.
                    ..#####.##..#..##....#..#...#.
                    ......#.......##.##.#....##..#
                    ..#..##..#.###.....#.#..###.#.
                    ..#..#..##..#.#.##..###.......
                    ...##....#.##.#.#..##.##.#...#
                    .##...#.#.##..#.#........#.#..
                    .##...##.##..#.#.##.#.#.#.##.#
                    #..##....#...###.#..##.#...##.
                    .###.###..##......#..#...###.#
                    .#..#.####.#..#....#.##..#.#.#
                    ..#...#..#.#######....###.....
                    ####..#.#.#...##...##....#..##
                    ##..#.##.#.#..##.###.#.##.##..
                    ..#.........#.#.#.#.......#..#
                    ...##.#.....#.#.##........#..#
                    ##..###.....#.............#.##
                    .#...#....#..####.#.#......##.
                    ..#..##..###...#.....#...##..#
                    ...####..#.#.##..#....#.#.....
                    ####.#####.#.#....#.#....##.#.
                    #.#..#......#.........##..#.#.
                    #....##.....#........#..##.##.
                    .###.##...##..#.##.#.#...#.#.#
                    ##.###....##....#.#.....#.###.
                    ..#...#......#........####..#.
                    #....#.###.##.#...#.#.#.#.....
                    .........##....#...#.....#..##
                    ###....#.........#..#..#.#.#..
                    ##...#...###.#..#.###....#.##."},
286, 504)]

    fn partone_test(#[case] input: &str, #[case] expected1: usize, #[case] expected2: usize) {
        let (p1, p2) = run(input.to_string());
        assert_eq!(expected1, p1);
        assert_eq!(expected2, p2);
    }
}
