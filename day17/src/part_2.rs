use std::fs;

pub fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input.txt");
    let mut lines = input.lines();

    // Read and discard Register lines. (Adjust if these don't match your input.)
    lines.next(); // Should be "Register A: ..."
    lines.next(); // "Register B: ..."
    lines.next(); // "Register C: ..."

    // Find the Program line.
    let mut program_line = None;
    for line in lines {
        if line.trim().starts_with("Program:") {
            program_line = Some(line);
            break;
        }
    }

    let program_line = program_line.expect("Could not find a line starting with 'Program:'");
    let program = parse_program(program_line);

    // The target output is the program itself
    let target_output = program.iter().map(|&v| v as i32).collect::<Vec<_>>();

    for initial_a in 1.. {
        let mut reg_a = initial_a;
        let mut reg_b = 0;
        let mut reg_c = 0;

        let mut ip = 0;
        let mut output = Vec::new();
        let mut instruction_count = 0;
        let max_instructions = 1_000_000; // Safety limit to avoid infinite loops

        while ip < program.len() && instruction_count < max_instructions {
            instruction_count += 1;
            let opcode = program[ip];
            if ip + 1 >= program.len() {
                // No operand available, halt
                break;
            }
            let operand = program[ip + 1];
            match opcode {
                0 => {
                    // adv: A = A / (2 ^ combo_operand)
                    let exponent = combo_operand_value(operand, reg_a, reg_b, reg_c);
                    if exponent < 0 || exponent > 31 {
                        break;
                    }
                    let denominator = 2_i32.pow(exponent as u32);
                    reg_a = if denominator == 0 {
                        0
                    } else {
                        reg_a / denominator
                    };
                    ip += 2;
                }
                1 => {
                    // bxl: B = B ^ literal_operand
                    reg_b ^= operand as i32;
                    ip += 2;
                }
                2 => {
                    // bst: B = combo_operand_value % 8
                    let value = combo_operand_value(operand, reg_a, reg_b, reg_c);
                    reg_b = value.rem_euclid(8);
                    ip += 2;
                }
                3 => {
                    // jnz: if A != 0, ip = literal_operand else ip += 2
                    if reg_a != 0 {
                        ip = operand;
                        if ip >= program.len() {
                            break;
                        }
                        continue;
                    } else {
                        ip += 2;
                    }
                }
                4 => {
                    // bxc: B = B ^ C (operand is ignored)
                    reg_b ^= reg_c;
                    ip += 2;
                }
                5 => {
                    // out: output = combo_operand_value % 8
                    let value = combo_operand_value(operand, reg_a, reg_b, reg_c).rem_euclid(8);
                    output.push(value);
                    ip += 2;
                }
                6 => {
                    // bdv: B = A / (2 ^ combo_operand)
                    let exponent = combo_operand_value(operand, reg_a, reg_b, reg_c);
                    if exponent < 0 || exponent > 31 {
                        break;
                    }
                    let denominator = 2_i32.pow(exponent as u32);
                    reg_b = if denominator == 0 {
                        0
                    } else {
                        reg_a / denominator
                    };
                    ip += 2;
                }
                7 => {
                    // cdv: C = A / (2 ^ combo_operand)
                    let exponent = combo_operand_value(operand, reg_a, reg_b, reg_c);
                    if exponent < 0 || exponent > 31 {
                        break;
                    }
                    let denominator = 2_i32.pow(exponent as u32);
                    reg_c = if denominator == 0 {
                        0
                    } else {
                        reg_a / denominator
                    };
                    ip += 2;
                }
                _ => {
                    // Invalid opcode
                    break;
                }
            }
        }

        // Check if output matches the program
        if output == target_output {
            println!("Minimum initial value for register A: {}", initial_a);
            break;
        }

        // If we hit max_instructions, just try the next initial_a
    }
}

fn parse_program(line: &str) -> Vec<usize> {
    // Ensure the line has the expected format "Program: X,Y,Z"
    line.split(": ")
        .nth(1)
        .expect(&format!("No program found after colon in line: {}", line))
        .split(',')
        .map(|s| s.parse::<usize>().expect("Invalid number in program"))
        .collect()
}

fn combo_operand_value(operand: usize, reg_a: i32, reg_b: i32, reg_c: i32) -> i32 {
    match operand {
        0..=3 => operand as i32,
        4 => reg_a,
        5 => reg_b,
        6 => reg_c,
        // 7 is reserved and won't appear in valid programs
        _ => 0,
    }
}
