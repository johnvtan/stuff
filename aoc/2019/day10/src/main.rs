use ordered_float::OrderedFloat;
use std::collections::{HashMap, HashSet};

type AsteroidMap = HashSet<(usize, usize)>;

fn parse(input: String) -> AsteroidMap {
    let mut asteroids = HashSet::new();
    let mut y = 0;
    for line in input.lines() {
        let mut x = 0;
        for c in line.chars() {
            if c == '#' {
                asteroids.insert((x, y));
            }
            x += 1;
        }
        y += 1;
    }
    asteroids
}

type AsteroidsBySlope = HashMap<(OrderedFloat<f32>, bool), Vec<(usize, usize)>>;

fn asteroids_by_slope(coord: (usize, usize), map: &AsteroidMap) -> AsteroidsBySlope {
    let mut slopes: HashMap<(OrderedFloat<f32>, bool), Vec<(usize, usize)>> = HashMap::new();
    for other in map.iter() {
        let (coord_x, coord_y) = coord;
        let (other_x, other_y) = other;
        let above = *other_y < coord_y;

        if *other_x == coord_x && *other_y == coord_y {
            continue;
        }

        if *other_x == coord_x {
            // vertical, just check above or below
            slopes
                .entry((OrderedFloat(f32::INFINITY), above))
                .and_modify(|coords| coords.push(*other))
                .or_insert(vec![*other]);

            // println!("\t{:?} -> (INF, {}), len={}", other, above, slopes.len());
            continue;
        } else if *other_y == coord_y {
            let left = *other_x < coord_x;
            slopes
                .entry((OrderedFloat(0.0), left))
                .and_modify(|coords| coords.push(*other))
                .or_insert(vec![*other]);

            // println!("\t{:?} -> ({}, {}), len={}", other, 0, left, slopes.len());
            continue;
        }

        let rise: f32 = (coord_y as f32) - (*other_y as f32);
        let run: f32 = (*other_x as f32) - (coord_x as f32);

        let slope = OrderedFloat(rise / run);

        slopes
            .entry((slope, above))
            .and_modify(|coords| coords.push(*other))
            .or_insert(vec![*other]);
        // println!("\t{:?} -> ({}, {}), len={}", other, slope, above, slopes.len());
    }

    slopes
}

fn count_visible(coord: (usize, usize), map: &AsteroidMap) -> usize {
    // has slope and whether the asteroid is above or below
    asteroids_by_slope(coord, map).len()
}

fn part1(map: &AsteroidMap) -> (usize, usize) {
    let mut max_visible = 0;
    let mut best = (0, 0);

    for coord in map.iter() {
        let visible = count_visible(*coord, &map);
        // println!("{} coord: {:?}, visible {}", count, coord, visible);
        if visible > max_visible {
            best = *coord;
            max_visible = visible;
        }
    }

    println!("part1: {}", max_visible);
    best
}

fn part2(coord: (usize, usize), map: &AsteroidMap, n: usize) {
    let mut slopes = asteroids_by_slope(coord, map);
    for asteroids in slopes.values_mut() {
        asteroids.sort_by(|(x1, y1), (x2, y2)| {
            let dist1 = x1.abs_diff(coord.0) + y1.abs_diff(coord.1);
            let dist2 = x2.abs_diff(coord.0) + y2.abs_diff(coord.1);
            dist1.cmp(&dist2)
        });
    }

    let mut ordered_slopes = vec![(OrderedFloat(f32::INFINITY), true)];
    let mut q1_slopes = slopes
        .clone()
        .into_keys()
        .filter(|(slope, above)| {
            *slope != OrderedFloat(f32::INFINITY) && *slope > OrderedFloat(0.0) && *above
        })
        .collect::<Vec<(OrderedFloat<f32>, bool)>>();
    q1_slopes.sort_by(|a, b| b.cmp(a));
    // println!("q1: {:?}\n", q1_slopes);

    ordered_slopes.extend(q1_slopes);

    ordered_slopes.push((OrderedFloat(0.0), false));

    let mut q2_slopes = slopes
        .clone()
        .into_keys()
        .filter(|(slope, above)| {
            *slope != OrderedFloat(f32::INFINITY) && *slope < OrderedFloat(0.0) && !*above
        })
        .collect::<Vec<(OrderedFloat<f32>, bool)>>();
    q2_slopes.sort_by(|a, b| b.cmp(a));

    // println!("q2: {:?}\n", q2_slopes);

    ordered_slopes.extend(q2_slopes);

    ordered_slopes.push((OrderedFloat(f32::INFINITY), false));
    let mut q3_slopes = slopes
        .clone()
        .into_keys()
        .filter(|(slope, above)| {
            *slope != OrderedFloat(f32::INFINITY) && *slope > OrderedFloat(0.0) && !*above
        })
        .collect::<Vec<(OrderedFloat<f32>, bool)>>();
    q3_slopes.sort_by(|a, b| b.cmp(a));

    // println!("q3: {:?}\n", q3_slopes);
    ordered_slopes.extend(q3_slopes);

    ordered_slopes.push((OrderedFloat(0.0), true));
    let mut q4_slopes = slopes
        .clone()
        .into_keys()
        .filter(|(slope, above)| {
            *slope != OrderedFloat(f32::INFINITY) && *slope < OrderedFloat(0.0) && *above
        })
        .collect::<Vec<(OrderedFloat<f32>, bool)>>();
    q4_slopes.sort_by(|a, b| b.cmp(a));

    // println!("q4: {:?}\n", q4_slopes);
    ordered_slopes.extend(q4_slopes);

    let mut slope_index = 0;
    let mut count = 0;
    loop {
        let mut asteroid = (0, 0);
        while let Some(asteroids) = slopes.get_mut(&ordered_slopes[slope_index]) {
            if asteroids.is_empty() {
                slope_index = (slope_index + 1) % ordered_slopes.len();
                continue;
            }

            asteroid = asteroids.remove(0);
            // println!("destroyed asteroid {:?} with slope {:?} at ({:?})", count + 1, ordered_slopes[slope_index], asteroid);
            break;
        }

        count += 1;
        if count == n {
            println!(
                "destroyed asteroid {:?}, {:?}",
                asteroid,
                asteroid.0 * 100 + asteroid.1
            );
            return;
        }
        slope_index = (slope_index + 1) % ordered_slopes.len();
    }
}

fn main() {
    let input = "
.#......#...#.....#..#......#..##..#
..#.......#..........#..##.##.......
##......#.#..#..#..##...#.##.###....
..#........#...........#.......##...
.##.....#.......#........#..#.#.....
.#...#...#.....#.##.......#...#....#
#...#..##....#....#......#..........
....#......#.#.....#..#...#......#..
......###.......#..........#.##.#...
#......#..#.....#..#......#..#..####
.##...##......##..#####.......##....
.....#...#.........#........#....#..
....##.....#...#........#.##..#....#
....#........#.###.#........#...#..#
....#..#.#.##....#.........#.....#.#
##....###....##..#..#........#......
.....#.#.........#.......#....#....#
.###.....#....#.#......#...##.##....
...##...##....##.........#...#......
.....#....##....#..#.#.#...##.#...#.
#...#.#.#.#..##.#...#..#..#..#......
......#...#...#.#.....#.#.....#.####
..........#..................#.#.##.
....#....#....#...#..#....#.....#...
.#####..####........#...............
#....#.#..#..#....##......#...#.....
...####....#..#......#.#...##.....#.
..##....#.###.##.#.##.#.....#......#
....#.####...#......###.....##......
.#.....#....#......#..#..#.#..#.....
..#.......#...#........#.##...#.....
#.....####.#..........#.#.......#...
..##..#..#.....#.#.........#..#.#.##
.........#..........##.#.##.......##
#..#.....#....#....#.#.......####..#
..............#.#...........##.#.#..
";

    let map = parse(input.trim().to_owned());
    println!("num asteroids: {}", map.len());
    let best_coord = part1(&map);
    println!("best coord {:?}", best_coord);
    part2(best_coord, &map, 200);
}
