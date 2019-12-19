use std::collections::HashMap;

struct Orbits {
    children: HashMap<String, Vec<String>>,
    parents: HashMap<String, Vec<String>>,
}

fn get_input() -> Orbits {
    let mut children: HashMap<String, Vec<String>> = HashMap::new();
    let mut parents: HashMap<String, Vec<String>> = HashMap::new();
    for x in std::fs::read_to_string("src/day6_input").unwrap().split("\n") {
        let x = x.split(")").collect::<Vec<&str>>();
        let parent = x[0].to_string();
        let child = x[1].to_string();
        match children.get_mut(&parent) {
            Some(e) => { e.push(child.clone()); },
            None => { children.insert(parent.clone(), vec!(child.clone())); },
        }
        match parents.get_mut(&child) {
            Some(e) => { e.push(parent.clone()); },
            None => { parents.insert(child.clone(), vec!(parent.clone())); },
        }
    }
    return Orbits { children, parents };
}

struct OrbitCount {
    num_children: i32,
    num_orbits: i32,
}

fn count_children(parent: &String, orbits: &Orbits) -> OrbitCount {
    let mut result = OrbitCount {
        num_children: 0,
        num_orbits: 0,
    };
    for child in orbits.children.get(parent).unwrap_or(&vec!()) {
        let count = count_children(child, orbits);
        result.num_children += count.num_children + 1;
        result.num_orbits += count.num_orbits + count.num_children + 1;
    }
    return result;
}

pub fn part1() {
    let orbits = get_input();
    let root = "COM".to_string();
    let result = count_children(&root, &orbits);
    println!("part 1 result = {}", result.num_orbits);
}

fn find_path(start: &String, end: &String, prev: &String, orbits: &Orbits) -> Option<i32> {
    if start.eq(end) {
        return Option::Some(0);
    }

    let mut result = Option::None;
    for child in orbits.children.get(start).unwrap_or(&vec!()) {
        if prev.eq(child) {
            continue;
        }
        let dist = find_path(child, end, start, orbits);
        if dist.is_some() && (result.is_none() || dist.unwrap() < result.unwrap()) {
            result = Option::Some(dist.unwrap() + 1);
        }
    }
    for parent in orbits.parents.get(start).unwrap_or(&vec!()) {
        if prev.eq(parent) {
            continue;
        }
        let dist = find_path(parent, end, start, orbits);
        if dist.is_some() && (result.is_none() || dist.unwrap() < result.unwrap()) {
            result = Option::Some(dist.unwrap() + 1);
        }
    }
    return result;
}

pub fn part2() {
    let orbits = get_input();
    let start = "YOU".to_string();
    let end = "SAN".to_string();
    let result = find_path(&start, &end, &start, &orbits);
    match result {
        Some(dist) => {
            println!("part 2 result = {}", dist - 2);
        },
        None => {
            println!("Unable to find path from {} to {}", start, end);
            std::process::exit(1);
        }
    }
}
