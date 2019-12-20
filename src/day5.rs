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

fn eval_int_code(user_input: i32, program: &mut Vec<i32>) -> Option<i32> {
    let mut input_used = false;
    let mut last_output = Option::None;
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
                if input_used {
                    println!("Tried to get input twice");
                    std::process::exit(1);
                }
                input_used = true;
                let output_index = program[next_op_index + 1];
                program[output_index as usize] = user_input;
                next_op_index += 2;
            },
            4 => {
                last_output = Option::Some(resolve_input(
                    (instruction % 1000) / 100,
                    program[next_op_index + 1],
                    program));
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
                return last_output;
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
    println!("part 1 result = {}", eval_int_code(1, &mut program.clone()).unwrap());
    println!("part 2 result = {}", eval_int_code(5, &mut program.clone()).unwrap());
}
