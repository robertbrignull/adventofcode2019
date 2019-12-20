mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("Usage: cargo run --release (day number)");
        std::process::exit(1);
    }

    let day_number_arg = args.get(1).unwrap();
    let day_number = day_number_arg.parse::<i32>();
    if !day_number.is_ok() {
        println!("Unable to parse {} as a day number", day_number_arg);
        std::process::exit(1);
    }
    let day_number = day_number.unwrap();

    match day_number {
        1 => {
            println!("Results for day {}...", day_number);
            day1::part1();
            day1::part2();
        },
        2 => {
            println!("Results for day {}...", day_number);
            day2::part1();
            day2::part2();
        },
        3 => {
            println!("Results for day {}...", day_number);
            day3::run();
        },
        4 => {
            println!("Results for day {}...", day_number);
            day4::part1();
            day4::part2();
        },
        5 => {
            println!("Results for day {}...", day_number);
            day5::run();
        },
        6 => {
            println!("Results for day {}...", day_number);
            day6::part1();
            day6::part2();
        },
        _ => {
            println!("Unexpected day number: {}", day_number);
            std::process::exit(1);
        }
    }
}
