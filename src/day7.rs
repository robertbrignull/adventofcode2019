fn get_input() -> Vec<i32> {
    return std::fs::read_to_string("src/day7_input").unwrap()
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
}

fn get_all_phase_settings(prefix: Vec<i32>) -> Vec<Vec<i32>> {
    if prefix.len() == 5 {
        return vec!(prefix);
    }

    let mut result = Vec::new();
    for i in 0.. 5 {
        if !prefix.contains(&i) {
            let mut new_prefix = Vec::new();
            new_prefix.extend(&prefix);
            new_prefix.push(i);
            result.extend(get_all_phase_settings(new_prefix));
        }
    }
    return result;
}

fn resolve_input(mode: i32, input: i32, program: &Vec<i32>) -> i32 {
    if mode == 0 {
        return program[input as usize];
    } else {
        return input;
    }
}

fn eval_int_code(phase_setting: i32, input: i32, program: &mut Vec<i32>) -> i32 {
    let mut next_op_index = 0;
    let mut num_inputs = 0;
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
                let input = match num_inputs {
                    0 => phase_setting,
                    1 => input,
                    _ => {
                        println!("Asked for more than two inputs");
                        std::process::exit(1);
                    }
                };
                num_inputs += 1;
                let output_index = program[next_op_index + 1];
                program[output_index as usize] = input;
                next_op_index += 2;
            },
            4 => {
                return resolve_input(
                    (instruction % 1000) / 100,
                    program[next_op_index + 1],
                    program);
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
                println!("Reached instruction 99 without outputting");
                std::process::exit(1);
            },
            _ => {
                println!("Encountered unexpected op code {} at position {}", op, next_op_index);
                std::process::exit(1);
            }
        }
    }
}

pub fn run() {
    let initial_program = get_input();
    let all_phase_settings = get_all_phase_settings(vec!());
    let mut best_output: Option<i32> = Option::None;
    for phase_settings in all_phase_settings {
        let mut output = 0;
        for phase_setting in phase_settings {
            let mut program = initial_program.clone();
            output = eval_int_code(phase_setting, output, &mut program);
        }

        if best_output.is_none() || output > best_output.unwrap() {
            best_output = Option::Some(output);
        }
    }
    println!("part 1 result = {}", best_output.unwrap());
}
