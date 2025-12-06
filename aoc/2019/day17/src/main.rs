use intcode::*;
use std::collections::{HashSet, VecDeque};

struct Map {
    map: Vec<String>,
    num_rows: usize,
    num_cols: usize,
}

type Route = Vec<String>;

fn route_encoding_len(route: &Route) -> usize {
    route.iter().map(|x| x.len() + 1).sum::<usize>()
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Debug)]
struct Position {
    r: usize,
    c: usize,
    dir: Direction,
}

#[derive(Clone)]
struct RouteBuilder {
    route: Route,
    pos: Position,
    to_visit: Vec<Vec<bool>>,
    num_to_visit: usize,
}

struct RoutePlan {
    main_routine: Route,
    routine_a: Option<Route>,
    routine_b: Option<Route>,
    routine_c: Option<Route>,
}

impl Map {
    fn new(map: String) -> Self {
        let map: Vec<String> = map.trim().split('\n').map(|x| x.to_string()).collect();

        let num_cols = map[0].len();
        let num_rows = map.len();
        Self {
            map,
            num_rows,
            num_cols,
        }
    }

    fn at(&self, r: usize, c: usize) -> char {
        self.map[r].chars().nth(c).unwrap()
    }

    fn get_starting_pos(&self) -> Position {
        for r in 0..self.num_rows {
            for c in 0..self.num_cols {
                if self.at(r, c) == '^' {
                    return Position {
                        r,
                        c,
                        dir: Direction::Up,
                    };
                }
            }
        }
        panic!("could not find");
    }

    fn render(&self) {
        for line in self.map.iter() {
            println!("{}", line);
        }
    }

    fn find_intersections(&self) -> Vec<(usize, usize)> {
        let mut intersections = vec![];

        for r in 1..self.num_rows - 1 {
            for c in 1..self.num_cols - 1 {
                if self.at(r, c) != '#' {
                    continue;
                }
                let left = self.at(r, c - 1) == '#';
                let right = self.at(r, c + 1) == '#';
                let up = self.at(r - 1, c) == '#';
                let down = self.at(r + 1, c) == '#';
                if left && right && up && down {
                    intersections.push((r, c));
                }
            }
        }

        println!("{:?}", intersections);
        intersections
    }

    fn to_boolean_vec(&self) -> (Vec<Vec<bool>>, usize) {
        let mut n = 0;
        let mut v = vec![vec![false; self.num_cols]; self.num_rows];
        for r in 0..self.num_rows {
            for c in 0..self.num_cols {
                if self.at(r, c) == '#' {
                    v[r][c] = true;
                    n += 1;
                }
            }
        }
        (v, n)
    }

    fn should_explore(&self, r: usize, c: usize, to_visit: &Vec<Vec<bool>>) -> bool {
        self.at(r, c) == '#' && to_visit[r][c]
    }

    fn directions_to_explore(&self, pos: &Position, to_visit: &Vec<Vec<bool>>) -> Vec<Direction> {
        let mut to_explore = vec![];
        let Position { r, c, dir: _ } = *pos;
        if r > 0 && self.should_explore(r - 1, c, &to_visit) {
            to_explore.push(Direction::Up);
        } else if r < self.num_rows - 1 && self.should_explore(r + 1, c, &to_visit) {
            to_explore.push(Direction::Down);
        } else if c > 0 && self.should_explore(r, c - 1, &to_visit) {
            to_explore.push(Direction::Left);
        } else if c < self.num_cols - 1 && self.should_explore(r, c + 1, &to_visit) {
            to_explore.push(Direction::Right);
        }
        // println!("From (r={}, c={}), explore {:?}", pos.r, pos.c, to_explore);
        to_explore
    }

    fn explore_segment(&self, route: &mut RouteBuilder, target_dir: Direction) {
        // first, rotate into position.
        match (route.pos.dir, target_dir) {
            (x, y) if x == y => {}
            (Direction::Up, Direction::Right)
            | (Direction::Right, Direction::Down)
            | (Direction::Down, Direction::Left)
            | (Direction::Left, Direction::Up) => {
                route.route.push("R".to_string());
            }
            (Direction::Right, Direction::Up)
            | (Direction::Down, Direction::Right)
            | (Direction::Left, Direction::Down)
            | (Direction::Up, Direction::Left) => {
                route.route.push("L".to_string());
            }
            other => unreachable!("invalid dir combo {:?}", other),
        }

        route.pos.dir = target_dir;

        // then go until the end of the segment.

        let mut dist = 0;
        match target_dir {
            Direction::Up => {
                assert!(route.pos.r >= 1);
                for new_r in (0..=route.pos.r - 1).rev() {
                    if self.at(new_r, route.pos.c) != '#' {
                        break;
                    }
                    // assert!(route.to_visit[new_r][route.pos.c], "new_r={}, c={}", new_r, route.pos.c);
                    if route.to_visit[new_r][route.pos.c] {
                        route.to_visit[new_r][route.pos.c] = false;
                        route.num_to_visit -= 1;
                    }
                    route.pos.r = new_r;
                    dist += 1;
                }
            }
            Direction::Down => {
                assert!(route.pos.r < self.num_rows - 1);
                for new_r in route.pos.r + 1..self.num_rows {
                    if self.at(new_r, route.pos.c) != '#' {
                        break;
                    }
                    // assert!(route.to_visit[new_r][route.pos.c], "new_r={}, c={}", new_r, route.pos.c);
                    if route.to_visit[new_r][route.pos.c] {
                        route.to_visit[new_r][route.pos.c] = false;
                        route.num_to_visit -= 1;
                    }
                    route.pos.r = new_r;
                    dist += 1;
                }
            }
            Direction::Left => {
                assert!(route.pos.c > 0);
                for new_c in (0..=route.pos.c - 1).rev() {
                    if self.at(route.pos.r, new_c) != '#' {
                        break;
                    }
                    // assert!(route.to_visit[route.pos.r][new_c], "r={}, new_c={}", route.pos.r, new_c);
                    if route.to_visit[route.pos.r][new_c] {
                        route.to_visit[route.pos.r][new_c] = false;
                        route.num_to_visit -= 1;
                    }
                    route.pos.c = new_c;
                    dist += 1;
                }
            }
            Direction::Right => {
                assert!(route.pos.c < self.num_cols - 1);
                for new_c in route.pos.c + 1..self.num_cols {
                    if self.at(route.pos.r, new_c) != '#' {
                        break;
                    }
                    // assert!(route.to_visit[route.pos.r][new_c], "r={}, new_c={}", route.pos.r, new_c);
                    if route.to_visit[route.pos.r][new_c] {
                        route.to_visit[route.pos.r][new_c] = false;
                        route.num_to_visit -= 1;
                    }
                    route.pos.c = new_c;
                    dist += 1;
                }
            }
        }
        route.route.push(dist.to_string());
    }

    fn explore_route(&self, curr: &mut RouteBuilder) -> Vec<RouteBuilder> {
        let pos = &curr.pos;
        // println!("explore_route: {:?} {:?}", curr.route, curr.pos);
        assert!(self.at(pos.r, pos.c) == '#' || self.at(pos.r, pos.c) == '^');

        let to_explore = self.directions_to_explore(pos, &curr.to_visit);
        assert!(to_explore.len() > 0 || curr.num_to_visit == 0);

        let mut first = true;

        let mut new_branches = vec![];
        for dir in to_explore {
            if first {
                self.explore_segment(curr, dir);
                first = false;
            } else {
                let mut branch = curr.clone();
                self.explore_segment(&mut branch, dir);
                new_branches.push(branch);
            }
        }
        new_branches
    }

    fn generate_all_routes(&self) -> Vec<Route> {
        let (to_visit, num_to_visit) = self.to_boolean_vec();
        let mut routes_in_progress = VecDeque::from(vec![RouteBuilder {
            route: vec![],
            pos: self.get_starting_pos(),
            to_visit,
            num_to_visit,
        }]);

        let mut finished_routes = vec![];

        while routes_in_progress.len() > 0 {
            if routes_in_progress.front().unwrap().num_to_visit == 0 {
                let r = routes_in_progress.pop_front().unwrap();
                finished_routes.push(r.route);
                continue;
            }

            let new_branches = self.explore_route(routes_in_progress.front_mut().unwrap());
            for branch in new_branches {
                routes_in_progress.push_back(branch);
            }
        }

        finished_routes
    }

    fn compress_one_greedy2(&self, route: &Route, plan_name: &str) -> (Route, Route) {
        println!("to_compress");
        for (i, r) in route.iter().enumerate() {
            print!("({i}, {r}) ");
        }
        println!("");

        let mut plan = vec![route[0].clone(), route[1].clone()];
        let mut candidates = HashSet::new();
        let mut upper_bound = usize::max_value();
        for i in 2..route.len() - 1 {
            if route[i] == plan[0] && route[i + 1] == plan[1] {
                candidates.insert(i);
                upper_bound = std::cmp::min(upper_bound, i);
            }
        }

        assert!(upper_bound > 2);
        let mut i = 2;
        while i < route.len() - 1 {
            if candidates.contains(&i) {
                // pray it's not tricky
                break;
            }

            let next_chunk = vec![route[i].clone(), route[i + 1].clone()];
            let next_chunk_len = route_encoding_len(&next_chunk);

            println!(
                "plan len {} next_chunk_len {} plan {:?}",
                route_encoding_len(&plan),
                next_chunk_len,
                plan
            );
            if route_encoding_len(&plan) + next_chunk_len > 20 {
                println!(
                    "too long, exit now: plan is {:?}, len= {:?} + {:?}",
                    plan,
                    route_encoding_len(&plan),
                    next_chunk_len,
                );
                break;
            }

            if route[i] == "A" || route[i] == "B" || route[i] == "C" {
                // hack lol
                break;
            }

            let mut next_candidates = HashSet::new();
            for cand in candidates.iter() {
                if cand + i + 1 >= route.len() {
                    continue;
                }

                println!(
                    "\tcandidate at {} shows {}{} (expect {}{})",
                    cand,
                    route[cand + i],
                    route[cand + i + 1],
                    route[i],
                    route[i + 1],
                );

                if route[cand + i] != route[i] || route[cand + i + 1] != route[i + 1] {
                    println!("\t\tremoved {}", cand);
                    continue;
                }

                next_candidates.insert(*cand);
            }

            println!(
                "pattern len {i}, next candidates len {}",
                next_candidates.len()
            );
            if next_candidates.len() == 0 {
                println!("exiting, plan is {:?}", &route[0..i]);
                break;
            }

            plan.push(route[i].clone());
            plan.push(route[i + 1].clone());
            candidates = next_candidates;
            i += 2;
            println!(
                "continuing: plan {:?}, i {i}, upper {upper_bound}",
                &route[0..i]
            );
        }

        let mut compressed_route = vec![];
        candidates.insert(0);
        i = 0;
        while i < route.len() {
            if candidates.contains(&i) {
                compressed_route.push(plan_name.to_string());
                i += plan.len();
            } else {
                compressed_route.push(route[i].clone());
                i += 1;
            }
        }

        println!(
            "old len {}, compressed len {}, plan len {}",
            route.len(),
            compressed_route.len(),
            plan.len()
        );
        println!("compressed {:?}, plan {:?}", compressed_route, plan);
        (compressed_route, plan)
    }

    // returns the compressed route and the route plan
    fn compress_one_greedy(&self, route: &Route, plan_name: &str) -> (Route, Route) {
        println!("to_compress");
        for (i, r) in route.iter().enumerate() {
            print!("({i}, {r}) ");
        }
        println!("");

        let mut plan = vec![route[0].clone(), route[1].clone()];
        let mut candidates = HashSet::new();
        for i in 2..route.len() - 1 {
            if route[i] == plan[0] && route[i + 1] == plan[1] {
                candidates.insert(i);
            }
        }

        // try to extend the pattern but only so that the total number of items compressed
        // increases.
        let mut pattern_len = 1;
        for i in 2..route.len() - 1 {
            if candidates.contains(&i) {
                // pray it's not tricky
                break;
            }

            // enforce maximum encoding length
            if route_encoding_len(&plan) + route[i].len() > 20 {
                println!(
                    "too long, exit now: plan is {:?}, len= {:?} + {:?}",
                    plan,
                    route_encoding_len(&plan),
                    route[i].len()
                );
                break;
            }

            if route[i] == "A" || route[i] == "B" || route[i] == "C" {
                // hack lol
                break;
            }

            pattern_len += 1;

            let mut next_candidates = HashSet::new();
            for cand in candidates.iter() {
                if cand + pattern_len >= route.len() {
                    continue;
                }

                println!(
                    "\tcandidate at {} shows {} (expect {})",
                    cand,
                    route[cand + pattern_len],
                    route[pattern_len]
                );
                if route[cand + pattern_len] != route[pattern_len] {
                    println!("\t\tremoved {}", cand);
                    continue;
                }

                next_candidates.insert(*cand);
            }

            println!(
                "pattern len {pattern_len}, next candidates len {}",
                next_candidates.len()
            );
            println!("plan {:?}", &route[0..pattern_len]);
            if next_candidates.len() == 0 {
                break;
            }

            plan.push(route[i].clone());
            candidates = next_candidates;
        }

        let mut compressed_route = vec![];
        candidates.insert(0);

        let mut i = 0;
        while i < route.len() {
            if candidates.contains(&i) {
                compressed_route.push(plan_name.to_string());
                i += plan.len();
            } else {
                compressed_route.push(route[i].clone());
                i += 1;
            }
        }

        println!(
            "old len {}, compressed len {}, plan len {}",
            route.len(),
            compressed_route.len(),
            plan.len()
        );
        println!("compressed {:?}, plan {:?}", compressed_route, plan);
        (compressed_route, plan)
    }

    // |route| is effectively a string. We basically want to find if we can break up the string
    // into 3 repeated patterns. It's sort of a compression problem?
    fn try_plan_route(&self, route: &Route) -> Option<RoutePlan> {
        let (route, route_a) = self.compress_one_greedy2(route, "A");
        let (route, route_b) = self.compress_one_greedy2(&route[1..].to_vec(), "B");
        let (route, route_c) = self.compress_one_greedy2(&route[2..].to_vec(), "C");

        // trust me
        let mut full_route = vec!["A".to_string(), "B".to_string(), "A".to_string()];

        for r in route {
            full_route.push(r);
        }

        println!("=================================");
        println!("full route: {:?}", full_route);
        println!("A: {:?}", route_a);
        println!("B: {:?}", route_b);
        println!("C: {:?}", route_c);

        Some(RoutePlan {
            main_routine: full_route,
            routine_a: Some(route_a),
            routine_b: Some(route_b),
            routine_c: Some(route_c),
        })
    }

    fn plan_route(&self) -> RoutePlan {
        // My plan for this is basically generate every possible path that covers every # in the
        // map. Then given these global plans, try to split it up into 3 plans and a main routine
        // that can cover the whole map. Remember, the goal is to find *any* plan that works.

        let possible_routes = self.generate_all_routes();
        for route in possible_routes {
            if let Some(plan) = self.try_plan_route(&route) {
                return plan;
            }
        }

        panic!("couldn't find route");
    }
}

fn commit_route(cpu: &mut Intcode, route: &Route) {
    for (i, r) in route.iter().enumerate() {
        for c in r.chars() {
            let num = c as i64;
            cpu.input.push_back(num);
        }

        if i == route.len() - 1 {
            cpu.input.push_back('\n' as i64);
        } else {
            cpu.input.push_back(',' as i64);
        }
    }
}

fn main() {
    let program = std::fs::read_to_string("program.txt").unwrap();
    let map = {
        let mut cpu = Intcode::new(csv_to_vec(program.clone()));

        cpu.run();
        String::from_utf8(cpu.output.iter().map(|x| *x as u8).collect::<Vec<u8>>()).unwrap()

        //     let test ="..#..........
        // ..#..........
        // #######...###
        // #.#...#...#.#
        // #############
        // ..#...#...#..
        // ..#####...^..".to_string();
    };

    let map = Map::new(map);
    map.render();

    let mut cpu = Intcode::new(csv_to_vec(program));
    cpu.write_memory(0, 2);

    println!("{}x{}", map.num_rows, map.num_cols);

    println!(
        "{:?}",
        map.find_intersections()
            .into_iter()
            .map(|coord| coord.0 * coord.1)
            .sum::<usize>()
    );

    let route = map.generate_all_routes();

    for (i, r) in route[0].iter().enumerate() {
        print!("({}, {:?}) ", i, r);
    }
    println!("");

    let full_plan = map.try_plan_route(&route[0]).unwrap();

    commit_route(&mut cpu, &full_plan.main_routine);
    commit_route(&mut cpu, &full_plan.routine_a.unwrap());
    commit_route(&mut cpu, &full_plan.routine_b.unwrap());
    commit_route(&mut cpu, &full_plan.routine_c.unwrap());

    cpu.input.push_back('n' as i64);
    cpu.input.push_back(10);

    println!("INPUT: =====");
    for i in cpu.input.iter() {
        print!("{}", *i as u8 as char);
    }
    print!("======\n");

    while !cpu.is_halted() {
        cpu.run();
        // let map  = String::from_utf8(cpu.output.clone().into_iter().map(|x| x as u8).collect::<Vec<u8>>()).unwrap();
        // let map = Map::new(map);

        // cpu.output.clear();
        // map.render();
    }
    println!("output len {:?}", cpu.output.len());
    println!("output: {:?}", cpu.output.pop_back().unwrap());
}
