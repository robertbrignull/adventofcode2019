use std::io::stdout;
use std::io::Write;

fn get_input() -> Vec<i32> {
    return std::fs::read_to_string("src/day5_input").unwrap()
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
}

fn resolve_input(mode: i32, input: i32, program: &Vec<i32>) -> i32 {
    if mode == 0 {
        return program[input as usize];
    } else {
        return input;
    }
}

fn read_int(prompt: String) -> i32 {
    print!("{}", prompt);
    stdout().flush().unwrap();

    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("failed to read from stdin");

    return match input.trim().parse::<i32>() {
        Ok(i) => i,
        Err(..) => {
            println!("User input \"{}\" is not a valid integer", input);
            std::process::exit(1);
        },
    };
}

fn eval_int_code(part_number: &str, program: &mut Vec<i32>) {
    let mut next_op_index = 0;
    loop {
        let instruction = program[next_op_index];
        let op = instruction % 100;
        match op {
            1 | 2 => {
                let input_1 = resolve_input(
                    (instruction % 1000) / 100,
                    program[next_op_index + 1],
                    program);
                let input_2 = resolve_input(
                    (instruction % 10000) / 1000,
                    program[next_op_index + 2],
                    program);

                let output = if op == 1 {
                    input_1 + input_2
                } else {
                    input_1 * input_2
                };

                let output_index = program[next_op_index + 3];
                program[output_index as usize] = output;
                next_op_index += 4;
            },
            3 => {
                let input = read_int(format!("Enter input for {}: ", part_number));
                let output_index = program[next_op_index + 1];
                program[output_index as usize] = input;
                next_op_index += 2;
            },
            4 => {
                let input = resolve_input(
                    (instruction % 1000) / 100,
                    program[next_op_index + 1],
                    program);
                println!("{}", input);
                next_op_index += 2;
            },
            5 | 6 => {
                let input_1 = resolve_input(
                    (instruction % 1000) / 100,
                    program[next_op_index + 1],
                    program);
                let input_2 = resolve_input(
                    (instruction % 10000) / 1000,
                    program[next_op_index + 2],
                    program);

                if op == 5 && input_1 != 0 {
                    next_op_index = input_2 as usize;
                } else if op == 6 && input_1 == 0 {
                    next_op_index = input_2 as usize;
                } else {
                    next_op_index += 3;
                }
            },
            7 | 8 => {
                let input_1 = resolve_input(
                    (instruction % 1000) / 100,
                    program[next_op_index + 1],
                    program);
                let input_2 = resolve_input(
                    (instruction % 10000) / 1000,
                    program[next_op_index + 2],
                    program);

                let output = if op == 7 && input_1 < input_2 {
                    1
                } else if op == 8 && input_1 == input_2 {
                    1
                } else {
                    0
                };

                let output_index = program[next_op_index + 3];
                program[output_index as usize] = output;
                next_op_index += 4;
            },
            99 => {
                return;
            },
            _ => {
                println!("Encountered unexpected op code {} at position {}", op, next_op_index);
                std::process::exit(1);
            }
        }
    }
}

pub fn run() {
    let program = get_input();
    println!("Running part 1...");
    eval_int_code("part 1", &mut program.clone());
    println!("Running part 2...");
    eval_int_code("part 2", &mut program.clone());
}
