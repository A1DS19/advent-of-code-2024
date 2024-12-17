use std::fs;

pub mod part_2;
use part_2::main as part_2_main;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input.txt");
    let mut lines = input.lines();

    let mut reg_a = parse_register(lines.next().unwrap());
    let mut reg_b = parse_register(lines.next().unwrap());
    let mut reg_c = parse_register(lines.next().unwrap());
    lines.next(); // Skip empty line
    let program_line = lines.next().unwrap();
    let program = parse_program(program_line);

    let mut ip = 0;
    let mut output = Vec::new();

    while ip < program.len() {
        let opcode = program[ip];
        if ip + 1 >= program.len() {
            break;
        }
        let operand = program[ip + 1];
        match opcode {
            0 => {
                // adv: reg_a = reg_a / (2 ^ combo_operand)
                let exponent = combo_operand_value(operand, reg_a, reg_b, reg_c);
                let denominator = 2_i32.pow(exponent as u32);
                if denominator != 0 {
                    reg_a /= denominator;
                } else {
                    reg_a = 0;
                }
                ip += 2;
            }
            1 => {
                // bxl: reg_b ^= literal_operand
                reg_b ^= operand as i32;
                ip += 2;
            }
            2 => {
                // bst: reg_b = combo_operand % 8
                reg_b = combo_operand_value(operand, reg_a, reg_b, reg_c) % 8;
                ip += 2;
            }
            3 => {
                // jnz: if reg_a != 0, ip = literal_operand
                if reg_a != 0 {
                    ip = operand;
                } else {
                    ip += 2;
                }
            }
            4 => {
                // bxc: reg_b ^= reg_c
                reg_b ^= reg_c;
                ip += 2;
            }
            5 => {
                // out: output combo_operand % 8
                let value = combo_operand_value(operand, reg_a, reg_b, reg_c) % 8;
                output.push(value);
                ip += 2;
            }
            6 => {
                // bdv: reg_b = reg_a / (2 ^ combo_operand)
                let exponent = combo_operand_value(operand, reg_a, reg_b, reg_c);
                let denominator = 2_i32.pow(exponent as u32);
                if denominator != 0 {
                    reg_b = reg_a / denominator;
                } else {
                    reg_b = 0;
                }
                ip += 2;
            }
            7 => {
                // cdv: reg_c = reg_a / (2 ^ combo_operand)
                let exponent = combo_operand_value(operand, reg_a, reg_b, reg_c);
                let denominator = 2_i32.pow(exponent as u32);
                if denominator != 0 {
                    reg_c = reg_a / denominator;
                } else {
                    reg_c = 0;
                }
                ip += 2;
            }
            _ => {
                break;
            }
        }
    }

    let result = output
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<_>>()
        .join(",");

    println!("part 1: {}", result);
    part_2_main();
}

fn parse_register(line: &str) -> i32 {
    line.split(": ").nth(1).unwrap().parse().unwrap()
}

fn parse_program(line: &str) -> Vec<usize> {
    line.split(": ")
        .nth(1)
        .unwrap()
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect()
}

fn combo_operand_value(operand: usize, reg_a: i32, reg_b: i32, reg_c: i32) -> i32 {
    match operand {
        0..=3 => operand as i32,
        4 => reg_a,
        5 => reg_b,
        6 => reg_c,
        _ => panic!("Invalid combo operand"),
    }
}
