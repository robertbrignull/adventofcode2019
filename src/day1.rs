fn get_input() -> Vec<i32> {
    return std::fs::read_to_string("src/day1_input").unwrap()
        .split("\n")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
}

fn calc_fuel_for_mass(mass: &i32) -> i32 {
    return (mass / 3) - 2;
}

fn calc_recursive_fuel_for_mass(mass: &i32) -> i32 {
    let mut fuel: i32 = 0;
    let mut extra_fuel: i32 = calc_fuel_for_mass(mass);
    while extra_fuel > 0 {
        fuel += extra_fuel;
        extra_fuel = calc_fuel_for_mass(&extra_fuel);
    }
    return fuel;
}


pub fn run() {
    let input = get_input();

    let part1_result: i32 = input.iter()
        .map(|x| calc_fuel_for_mass(x))
        .sum();
    println!("part 1 result = {}", part1_result);

    let part2_result: i32 = input.iter()
        .map(|x| calc_recursive_fuel_for_mass(x))
        .sum();
    println!("part 2 result = {}", part2_result);
}
