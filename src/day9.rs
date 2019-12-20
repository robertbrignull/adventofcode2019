#[derive(Clone)]
struct Program {
    next_op_index: usize,
    relative_base: i64,
    memory: Vec<i64>,
}

#[derive(Debug)]
enum ProgramResult {
    NeedsInput,
    Output(i64),
    Terminated,
}

impl Program {
    fn get(&self, index: usize) -> i64 {
        if index < self.memory.len() {
            return self.memory[index];
        } else {
            return 0;
        }
    }

    fn get_param(&self, param_number: usize) -> i64 {
        let instruction = self.get(self.next_op_index);
        let mode = (instruction / (10 as i64).pow(param_number as u32 + 2)) % 10;
        let input = self.get(self.next_op_index + 1 + param_number);
        return match mode {
            0 => self.get(input as usize),
            1 => input,
            2 => self.get((input + self.relative_base) as usize),
            _ => {
                println!("Encountered unexpected mode {}", mode);
                std::process::exit(1);
            }
        };
    }

    fn put_param(&mut self, param_number: usize, value: i64) {
        let instruction = self.get(self.next_op_index);
        let mode = (instruction / (10 as i64).pow(param_number as u32 + 2)) % 10;
        let input = self.get(self.next_op_index + 1 + param_number);
        let index = match mode {
            0 => input as usize,
            2 => (input + self.relative_base) as usize,
            _ => {
                println!("Encountered unexpected output mode {}", mode);
                std::process::exit(1);
            }
        };
        if index >= self.memory.len() {
            let new_len = (index + 1).max(self.memory.len() * 2);
            self.memory.resize(new_len, 0);
        }
        self.memory[index] = value;
    }

    pub fn eval(&mut self, input: i64) -> ProgramResult {
        let mut input_used = false;
        loop {
            let instruction = self.get(self.next_op_index);
            let op = instruction % 100;
            match op {
                1 | 2 => {
                    let input_1 = self.get_param(0);
                    let input_2 = self.get_param(1);

                    let output = if op == 1 {
                        input_1 + input_2
                    } else {
                        input_1 * input_2
                    };

                    self.put_param(2, output);
                    self.next_op_index += 4;
                },
                3 => {
                    if input_used {
                        return ProgramResult::NeedsInput;
                    }
                    input_used = true;
                    self.put_param(0, input);
                    self.next_op_index += 2;
                },
                4 => {
                    let output = self.get_param(0);
                    self.next_op_index += 2;
                    return ProgramResult::Output(output);
                },
                5 | 6 => {
                    let input_1 = self.get_param(0);
                    let input_2 = self.get_param(1);

                    if op == 5 && input_1 != 0 {
                        self.next_op_index = input_2 as usize;
                    } else if op == 6 && input_1 == 0 {
                        self.next_op_index = input_2 as usize;
                    } else {
                        self.next_op_index += 3;
                    }
                },
                7 | 8 => {
                    let input_1 = self.get_param(0);
                    let input_2 = self.get_param(1);

                    let output = if op == 7 && input_1 < input_2 {
                        1
                    } else if op == 8 && input_1 == input_2 {
                        1
                    } else {
                        0
                    };

                    self.put_param(2, output);
                    self.next_op_index += 4;
                },
                9 => {
                    let input = self.get_param(0);
                    self.relative_base += input;
                    self.next_op_index += 2;
                },
                99 => {
                    return ProgramResult::Terminated;
                },
                _ => {
                    println!("Encountered unexpected op code {} at position {}", op, self.next_op_index);
                    std::process::exit(1);
                }
            }
        }
    }
}

fn get_input() -> Program {
    let memory = std::fs::read_to_string("src/day9_input").unwrap()
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    return Program {
        next_op_index: 0,
        relative_base: 0,
        memory
    };
}

fn part1() {
    let mut program = get_input();
    match program.eval(1) {
        ProgramResult::Output(x) => {
            println!("part 1 result = {}", x);
        },
        x => {
            println!("Terminated unexpectedly with {:?}", x);
            std::process::exit(1);
        }
    }
}

fn part2() {
    let mut program = get_input();
    match program.eval(2) {
        ProgramResult::Output(x) => {
            println!("part 1 result = {}", x);
        },
        x => {
            println!("Terminated unexpectedly with {:?}", x);
            std::process::exit(1);
        }
    }
}

pub fn run() {
    part1();
    part2();
}