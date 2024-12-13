use std::fs;

#[derive(Debug)]
struct ClawMachine {
    button_a: (i128, i128),
    button_b: (i128, i128),
    prize: (i128, i128),
}

fn gcd(a: i128, b: i128) -> i128 {
    if b == 0 {
        a.abs()
    } else {
        gcd(b, a % b)
    }
}

fn extended_gcd(a: i128, b: i128) -> (i128, i128, i128) {
    if b == 0 {
        (a.abs(), if a >= 0 { 1 } else { -1 }, 0)
    } else {
        let (g, x, y) = extended_gcd(b, a % b);
        (g, y, x - (a / b) * y)
    }
}

fn find_solution(machine: &ClawMachine) -> Option<(i128, i128)> {
    let (ax, ay) = machine.button_a;
    let (bx, by) = machine.button_b;
    let (px, py) = machine.prize;

    // Solve for x coordinates
    let dx = gcd(ax, bx);
    if px % dx != 0 {
        return None;
    }

    // Solve for y coordinates
    let dy = gcd(ay, by);
    if py % dy != 0 {
        return None;
    }

    let (_, mut x1, mut y1) = extended_gcd(ax, bx);
    let (_, mut x2, mut y2) = extended_gcd(ay, by);

    x1 *= px / dx;
    y1 *= px / dx;
    x2 *= py / dy;
    y2 *= py / dy;

    // Find common solution
    if x1 != x2 || y1 != y2 {
        return None;
    }

    // Adjust for minimum positive solution
    let tx = bx / dx;
    let ty = -ax / dx;
    let mut k = 0;

    while x1 + k * tx < 0 || y1 + k * ty < 0 {
        k += 1;
    }

    Some((x1 + k * tx, y1 + k * ty))
}

fn parse_input(input: &str) -> Vec<ClawMachine> {
    input
        .trim()
        .split("\n\n")
        .filter_map(|chunk| {
            let lines: Vec<&str> = chunk.lines().collect();
            if lines.len() != 3 {
                return None;
            }

            let mut nums = lines
                .iter()
                .map(|line| {
                    line.split(|c: char| !c.is_digit(10) && c != '-')
                        .filter_map(|n| n.parse::<i128>().ok())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();

            if nums.iter().all(|n| n.len() == 2) {
                Some(ClawMachine {
                    button_a: (nums[0][0], nums[0][1]),
                    button_b: (nums[1][0], nums[1][1]),
                    prize: (
                        nums[2][0] + 10_000_000_000_000,
                        nums[2][1] + 10_000_000_000_000,
                    ),
                })
            } else {
                None
            }
        })
        .collect()
}

fn calculate_tokens(a_presses: i128, b_presses: i128) -> i128 {
    3 * a_presses + b_presses
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input.txt");
    let machines = parse_input(&input);

    let mut total_tokens = 0;
    let mut winnable_prizes = 0;

    for machine in machines {
        if let Some((a, b)) = find_solution(&machine) {
            winnable_prizes += 1;
            total_tokens += calculate_tokens(a, b);
            println!("Found solution: A={}, B={}", a, b);
        }
    }

    println!("Winnable prizes: {}", winnable_prizes);
    println!("Total tokens needed: {}", total_tokens);
}
