mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 1 {
        for i in 1..26 {
            println!("Day {} results...", i);
            run_day(i);
        }
        return;
    }

    if args.len() == 2 {
        let day_number_arg = args.get(1).unwrap();
        let day_number = day_number_arg.parse::<i32>();
        if !day_number.is_ok() {
            println!("Unable to parse {} as a day number", day_number_arg);
            std::process::exit(1);
        }
        run_day(day_number.unwrap());
        return;
    }

    println!("Usage: cargo run --release [day number]");
    std::process::exit(1);
}

fn run_day(day_number: i32) {
    match day_number {
        1 => { day1::run(); },
        2 => { day2::run(); },
        3 => { day3::run(); },
        4 => { day4::run(); },
        5 => { day5::run(); },
        6 => { day6::run(); },
        7 => { day7::run(); },
        8 => { day8::run(); },
        _ => {}
    }
}
