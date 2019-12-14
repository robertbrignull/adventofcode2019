fn get_input() -> Vec<i32> {
    return std::fs::read_to_string("src/day1_input").unwrap()
        .split("\n")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
}

fn calc_fuel_for_mass(mass: i32) -> i32 {
    return (mass / 3) - 2;
}

pub fn part1() {
    let mut total: i32 = 0;
    for input in get_input() {
        total += calc_fuel_for_mass(input);
    }
    println!("part 1 result = {}", total);
}

fn calc_recursive_fuel_for_mass(mass: i32) -> i32 {
    let mut fuel: i32 = 0;
    let mut extra_fuel: i32 = calc_fuel_for_mass(mass);
    while extra_fuel > 0 {
        fuel += extra_fuel;
        extra_fuel = calc_fuel_for_mass(extra_fuel);
    }
    return fuel;
}

pub fn part2() {
    let mut total: i32 = 0;
    for input in get_input() {
        total += calc_recursive_fuel_for_mass(input);
    }
    println!("part 2 result = {}", total);
}
