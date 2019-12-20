fn get_input() -> Vec<i32> {
    return std::fs::read_to_string("src/day2_input").unwrap()
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
}

fn eval_int_code(program: &mut Vec<i32>) {
    let mut next_op_index = 0;
    loop {
        let op = program[next_op_index];
        match op {
            1 | 2 => {
                let input_1_index = program[next_op_index + 1];
                let input_1 = program[input_1_index as usize];
                let input_2_index = program[next_op_index + 2];
                let input_2 = program[input_2_index as usize];
                let output_index = program[next_op_index + 3];
                let output = if op == 1 {
                    input_1 + input_2
                } else {
                    input_1 * input_2
                };
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

fn part1() {
    let mut program = get_input();
    program[1] = 12;
    program[2] = 2;
    eval_int_code(&mut program);
    println!("part 1 result = {}", program[0]);
}

fn part2() {
    let initial_program = get_input();
    for x in 0..100 {
        for y in 0..100 {
            let mut program = initial_program.clone();
            program[1] = x;
            program[2] = y;
            eval_int_code(&mut program);

            if program[0] == 19690720 {
                println!("part 2 result = {}{}", x, y);
                return;
            }
        }
    }
    println!("Unable to solve part 2");
    std::process::exit(1);
}

pub fn run() {
    part1();
    part2();
}
