use util::Solution;

pub fn solve(input: &str) -> Solution {
    let (registers, program) = input.split_once("\n\n").unwrap();
    let registers = registers
        .lines()
        .map(|line| {
            line.trim_matches(|c: char| !c.is_ascii_digit())
                .parse::<u64>()
                .unwrap()
        })
        .collect::<Vec<_>>();

    let program = program
        .split(',')
        .map(|segment| {
            segment
                .trim_matches(|c: char| !c.is_ascii_digit())
                .parse::<u8>()
                .unwrap()
        })
        .collect::<Vec<_>>();

    let results = run_program(registers.try_into().unwrap(), &program);

    let mut part1 = String::new();
    part1.push_str(format!("{}", results[0]).as_str());
    for number in results.iter().skip(1) {
        part1.push_str(format!(",{number}").as_str());
    }

    let part2 = find_quine_input(&program, program.len() - 1, 0).unwrap();

    Solution::from((part1, part2))
}

fn find_quine_input(program: &[u8], cursor: usize, so_far: u64) -> Option<u64> {
    for candidate in 0..8 {
        let input_value = 8 * so_far + candidate;
        if run_program([input_value, 0, 0], program) == program[cursor..] {
            if cursor == 0 {
                return Some(input_value);
            }
            if let Some(ret) = find_quine_input(program, cursor - 1, input_value) {
                return Some(ret);
            }
        }
    }
    None
}

fn run_program(mut registers: [u64; 3], program: &[u8]) -> Vec<u8> {
    let mut ip = 0;
    let mut outputs = Vec::new();
    while ip < program.len() {
        let code = program[ip];
        let literal = program[ip + 1];
        match code {
            0 => {
                let numerator = registers[0];
                let denominator = 2u64.pow(get_combo(literal, registers) as u32);
                registers[0] = numerator / denominator;
            }
            1 => {
                registers[1] ^= literal as u64;
            }
            2 => {
                registers[1] = get_combo(literal, registers) % 8;
            }
            3 => {
                if registers[0] != 0 {
                    ip = literal as usize;
                    continue;
                }
            }
            4 => {
                registers[1] ^= registers[2];
            }
            5 => {
                let value = get_combo(literal, registers) % 8;
                outputs.push(value as u8);
            }
            6 => {
                let numerator = registers[0];
                let denominator = 2u64.pow(get_combo(literal, registers) as u32);
                registers[1] = numerator / denominator;
            }
            7 => {
                let numerator = registers[0];
                let denominator = 2u64.pow(get_combo(literal, registers) as u32);
                registers[2] = numerator / denominator;
            }
            _ => panic!(),
        }
        ip += 2;
    }
    outputs
}

fn get_combo(code: u8, registers: [u64; 3]) -> u64 {
    match code {
        0..=3 => code as u64,
        4 => registers[0],
        5 => registers[1],
        6 => registers[2],
        _ => panic!(),
    }
}
