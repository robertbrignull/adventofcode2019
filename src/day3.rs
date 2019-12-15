use std::str::FromStr;

struct Point {
    x: i32,
    y: i32,
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        return self.x == other.x && self.y == other.y;
    }
}

enum Instruction {
    U(i32),
    D(i32),
    L(i32),
    R(i32),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let n = s[1..].parse::<i32>();
        if !n.is_ok() {
            return Result::Err(());
        }
        let n = n.unwrap();

        return match s.chars().nth(0).unwrap() {
            'U' => Result::Ok(Instruction::U(n)),
            'D' => Result::Ok(Instruction::D(n)),
            'L' => Result::Ok(Instruction::L(n)),
            'R' => Result::Ok(Instruction::R(n)),
            _ => Result::Err(()),
        }
    }
}

fn get_input() -> Vec<Vec<Instruction>> {
    return std::fs::read_to_string("src/day3_input").unwrap()
        .split("\n")
        .map(|line| line.split(",")
            .map(|x| x.parse::<Instruction>().unwrap())
            .collect::<Vec<Instruction>>())
        .collect::<Vec<Vec<Instruction>>>();
}

fn get_touches_points(wire: &Vec<Instruction>) -> Vec<Point> {
    let mut x = 0;
    let mut y = 0;
    let mut touching_points = Vec::new();
    for instruction in wire {
        match instruction {
            Instruction::U(n) => {
                for i in 1..n+1 {
                    touching_points.push(Point {x, y: y + i});
                }
                y += n;
            },
            Instruction::D(n) => {
                for i in 1..n+1 {
                    touching_points.push(Point {x, y: y - i});
                }
                y -= n;
            },
            Instruction::L(n) => {
                for i in 1..n+1 {
                    touching_points.push(Point {x: x - i, y});
                }
                x -= n;
            },
            Instruction::R(n) => {
                for i in 1..n+1 {
                    touching_points.push(Point {x: x + i, y});
                }
                x += n;
            },
        }
    }
    return touching_points;
}

fn get_intersecting_points(wires: &Vec<Vec<Instruction>>) -> Vec<Point> {
    let mut intersecting_points = get_touches_points(&wires[0]);
    for wire in wires.iter().skip(1) {
        let touching_spaces = get_touches_points(wire);
        intersecting_points.retain(|x| touching_spaces.contains(x));
    }
    return intersecting_points;
}

fn dist(point: &Point) -> i32 {
    return point.x.abs() + point.y.abs();
}

fn nearest_dist(points: &Vec<Point>) -> i32 {
    let mut nearest_dist = dist(&points[0]);
    for point in points.iter().skip(1) {
        let d = dist(point);
        if d < nearest_dist {
            nearest_dist = d;
        }
    }
    return nearest_dist;
}

fn intersection_dist(wire: &Vec<Instruction>, point: &Point) -> i32 {
    let mut x = 0;
    let mut y = 0;
    let mut steps = 0;
    for instruction in wire {
        match instruction {
            Instruction::U(n) => {
                if x == point.x && y < point.y && y + n >= point.y {
                    return steps + point.y - y;
                }
                y += n;
                steps += n;
            },
            Instruction::D(n) => {
                if x == point.x && y > point.y && y - n <= point.y {
                    return steps + y - point.y;
                }
                y -= n;
                steps += n;
            },
            Instruction::L(n) => {
                if y == point.y && x > point.x && x - n <= point.x {
                    return steps + x - point.x;
                }
                x -= n;
                steps += n;
            },
            Instruction::R(n) => {
                if y == point.y && x < point.x && x + n >= point.x {
                    return steps + point.x - x;
                }
                x += n;
                steps += n;
            },
        }
    }
    return steps;
}

fn signal_delay_sum_for_wires(wires: &Vec<Vec<Instruction>>, point: &Point) -> i32 {
    let mut delay = 0;
    for wire in wires {
        delay += intersection_dist(&wire, point);
    }
    return delay;
}

fn shortest_signal_delay(wires: &Vec<Vec<Instruction>>, points: &Vec<Point>) -> i32 {
    let mut shortest_intersection = signal_delay_sum_for_wires(wires, &points[0]);
    for point in points.iter().skip(1) {
        let d = signal_delay_sum_for_wires(wires, point);
        if d < shortest_intersection {
            shortest_intersection = d;
        }
    }
    return shortest_intersection;
}

pub fn part1() {
    let wires = get_input();
    let intersecting_points = get_intersecting_points(&wires);
    println!("part 1 result = {}", nearest_dist(&intersecting_points));
}

pub fn part2() {
    let wires = get_input();
    let intersecting_points = get_intersecting_points(&wires);
    println!("part 2 result = {}", shortest_signal_delay(&wires, &intersecting_points));
}
