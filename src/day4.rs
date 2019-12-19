struct Range {
    lower: u32,
    upper: u32,
}

fn get_input() -> Range {
    let input = std::fs::read_to_string("src/day4_input").unwrap()
        .split("-")
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    return Range {
        lower: input[0],
        upper: input[1],
    };
}

fn are_all_digits(password: &Vec<u8>) -> bool {
    return password.iter().all(|d| d >= &0 && d <= &9);
}

fn password_is_in_range(range: &Range, password: &Vec<u8>) -> bool {
    let mut num: u32 = 0;
    for d in password {
        num = (num * 10) + (*d as u32);
    }

    return range.lower <= num && num <= range.upper;
}

fn has_adjacent_digits(password: &Vec<u8>) -> bool {
    let mut last = &password[0];
    for d in password.iter().skip(1) {
        if d == last {
            return true;
        }
        last = d;
    }
    return false;
}

fn has_precisely_two_adjacent_digits(password: &Vec<u8>) -> bool {
    let mut last = &password[0];
    let mut run_length = 1;
    for d in password.iter().skip(1) {
        if d == last {
            run_length += 1;
        } else {
            if run_length == 2 {
                return true;
            }
            last = d;
            run_length = 1;
        }
    }
    return run_length == 2;
}

fn is_increasing(password: &Vec<u8>) -> bool {
    let mut last = &password[0];
    for d in password.iter().skip(1) {
        if d < last {
            return false;
        }
        last = d;
    }
    return true;
}

fn is_valid_part1_password(range: &Range, password: &Vec<u8>) -> bool {
    return password.len() == 6 &&
        are_all_digits(password) &&
        password_is_in_range(range, password) &&
        has_adjacent_digits(password) &&
        is_increasing(password);
}

fn is_valid_part2_password(range: &Range, password: &Vec<u8>) -> bool {
    return password.len() == 6 &&
        are_all_digits(password) &&
        password_is_in_range(range, password) &&
        has_precisely_two_adjacent_digits(password) &&
        is_increasing(password);
}

fn get_all_passwords(range: &Range, prefix: Vec<u8>) -> Vec<Vec<u8>> {
    if prefix.len() == 6 {
        return vec!(prefix);
    }

    let mut result = Vec::new();
    if prefix.len() == 0 {
        let lower = range.lower.to_string().chars().next().unwrap().to_digit(10).unwrap();
        let upper = range.upper.to_string().chars().next().unwrap().to_digit(10).unwrap();
        for d in lower..upper {
            result.extend(get_all_passwords(range, vec!(d as u8)));
        }
    }
    else {
        let last = prefix[prefix.len() - 1];
        for d in last..10 {
            let mut new_prefix: Vec<u8> = vec!();
            new_prefix.extend(&prefix);
            new_prefix.push(d);
            result.extend(get_all_passwords(range, new_prefix));
        }
    }
    return result;
}

pub fn part1() {
    let range = get_input();
    let mut passwords = get_all_passwords(&range, vec!());
    passwords.retain(|p| is_valid_part1_password(&range, p));
    println!("part 1 result = {}", passwords.len());
}

pub fn part2() {
    let range = get_input();
    let mut passwords = get_all_passwords(&range, vec!());
    passwords.retain(|p| is_valid_part2_password(&range, p));
    println!("part 2 result = {}", passwords.len());
}
