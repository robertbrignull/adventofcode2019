#[derive(Clone)]
struct Program {
    next_op_index: usize,
    memory: Vec<i32>,
}

fn get_input() -> Program {
    let memory = std::fs::read_to_string("src/day7_input").unwrap()
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    return Program {
        next_op_index: 0,
        memory
    };
}

fn get_all_phase_settings(prefix: Vec<i32>, choices: &Vec<i32>) -> Vec<Vec<i32>> {
    if prefix.len() == 5 {
        return vec!(prefix);
    }

    let mut result = Vec::new();
    for i in choices {
        if !prefix.contains(&i) {
            let mut new_prefix = Vec::new();
            new_prefix.extend(&prefix);
            new_prefix.push(*i);
            result.extend(get_all_phase_settings(new_prefix, choices));
        }
    }
    return result;
}

fn resolve_input(mode: i32, input: i32, program: &Program) -> i32 {
    if mode == 0 {
        return program.memory[input as usize];
    } else {
        return input;
    }
}

#[derive(Debug)]
enum ProgramResult {
    NeedsInput,
    Output(i32),
    Terminated,
}

fn eval_int_code(input: i32, program: &mut Program) -> ProgramResult {
    let mut input_used = false;
    loop {
        let instruction = program.memory[program.next_op_index];
        let op = instruction % 100;
        match op {
            1 | 2 => {
                let input_1 = resolve_input(
                    (instruction % 1000) / 100,
                    program.memory[program.next_op_index + 1],
                    program);
                let input_2 = resolve_input(
                    (instruction % 10000) / 1000,
                    program.memory[program.next_op_index + 2],
                    program);

                let output = if op == 1 {
                    input_1 + input_2
                } else {
                    input_1 * input_2
                };

                let output_index = program.memory[program.next_op_index + 3];
                program.memory[output_index as usize] = output;
                program.next_op_index += 4;
            },
            3 => {
                if input_used {
                    return ProgramResult::NeedsInput;
                }
                input_used = true;
                let output_index = program.memory[program.next_op_index + 1];
                program.memory[output_index as usize] = input;
                program.next_op_index += 2;
            },
            4 => {
                let output = resolve_input(
                    (instruction % 1000) / 100,
                    program.memory[program.next_op_index + 1],
                    program);
                program.next_op_index += 2;
                return ProgramResult::Output(output);
            },
            5 | 6 => {
                let input_1 = resolve_input(
                    (instruction % 1000) / 100,
                    program.memory[program.next_op_index + 1],
                    program);
                let input_2 = resolve_input(
                    (instruction % 10000) / 1000,
                    program.memory[program.next_op_index + 2],
                    program);

                if op == 5 && input_1 != 0 {
                    program.next_op_index = input_2 as usize;
                } else if op == 6 && input_1 == 0 {
                    program.next_op_index = input_2 as usize;
                } else {
                    program.next_op_index += 3;
                }
            },
            7 | 8 => {
                let input_1 = resolve_input(
                    (instruction % 1000) / 100,
                    program.memory[program.next_op_index + 1],
                    program);
                let input_2 = resolve_input(
                    (instruction % 10000) / 1000,
                    program.memory[program.next_op_index + 2],
                    program);

                let output = if op == 7 && input_1 < input_2 {
                    1
                } else if op == 8 && input_1 == input_2 {
                    1
                } else {
                    0
                };

                let output_index = program.memory[program.next_op_index + 3];
                program.memory[output_index as usize] = output;
                program.next_op_index += 4;
            },
            99 => {
                return ProgramResult::Terminated;
            },
            _ => {
                println!("Encountered unexpected op code {} at position {}", op, program.next_op_index);
                std::process::exit(1);
            }
        }
    }
}

fn part1() {
    let initial_program = get_input();
    let all_phase_settings = get_all_phase_settings(vec!(), &vec!(0, 1, 2, 3, 4));
    let best_output = all_phase_settings.iter()
        .map(|phase_settings| {
            // Create all the programs
            let mut programs = vec!();
            for _i in 0..phase_settings.len() {
                programs.push(initial_program.clone());
            }

            // Feed each program their phase setting
            for i in 0..phase_settings.len() {
                let mut program = programs.get_mut(i).unwrap();
                let phase_setting = phase_settings.get(i).unwrap();
                match eval_int_code(*phase_setting, &mut program) {
                    ProgramResult::NeedsInput => {
                        // expected
                    },
                    x => {
                        println!("Terminated unexpectedly with {:?}", x);
                        std::process::exit(1);
                    }
                }
            }

            // Run each program once with another input
            let mut prev_output = 0;
            for i in 0..phase_settings.len() {
                let mut program = programs.get_mut(i).unwrap();
                match eval_int_code(prev_output, &mut program) {
                    ProgramResult::Output(x) => {
                        prev_output = x;
                    },
                    x => {
                        println!("Terminated unexpectedly with {:?}", x);
                        std::process::exit(1);
                    }
                }
            }
            return prev_output;
        })
        .max().unwrap();
    println!("part 1 result = {}", best_output);
}

fn part2() {
    let initial_program = get_input();
    let all_phase_settings = get_all_phase_settings(vec!(), &vec!(5, 6, 7, 8, 9));
    let best_output = all_phase_settings.iter()
        .map(|phase_settings| {
            // Create all the programs
            let mut programs = vec!();
            for _i in 0..5 {
                programs.push(initial_program.clone());
            }

            // Feed each program their phase setting
            for i in 0..5 {
                let mut program = programs.get_mut(i).unwrap();
                let phase_setting = phase_settings.get(i).unwrap();
                match eval_int_code(*phase_setting, &mut program) {
                    ProgramResult::NeedsInput => {
                        // expected
                    },
                    x => {
                        println!("Terminated unexpectedly with {:?}", x);
                        std::process::exit(1);
                    }
                }
            }

            // Run each program in turn until the 1st one terminates
            let mut prev_output = 0;
            loop {
                for i in 0..5 {
                    let mut program = programs.get_mut(i).unwrap();
                    match eval_int_code(prev_output, &mut program) {
                        ProgramResult::Output(x) => {
                            prev_output = x;
                        },
                        ProgramResult::Terminated => {
                            if i == 0 {
                                return prev_output;
                            } else {
                                println!("Program {} terminated unexpectedly", i);
                                std::process::exit(1);
                            }
                        }
                        x => {
                            println!("Terminated unexpectedly with {:?}", x);
                            std::process::exit(1);
                        }
                    }
                }
            }
        })
        .max().unwrap();
    println!("part 2 result = {}", best_output);
}

pub fn run() {
    part1();
    part2();
}