/*
a report only counts as safe if both of the following are true:
- The levels are either all increasing or all decreasing.
- Any two adjacent levels differ by at least one and at most three.

7 6 4 2 1: Safe because the levels are all decreasing by 1 or 2.
1 2 7 8 9: Unsafe because 2 7 is an increase of 5.
9 7 6 2 1: Unsafe because 6 2 is a decrease of 4.
1 3 2 4 5: Unsafe because 1 3 is increasing but 3 2 is decreasing.
8 6 4 4 1: Unsafe because 4 4 is neither an increase or a decrease.
1 3 6 7 9: Safe because the levels are all increasing by 1, 2, or 3.
*/

fn is_safe(report: &[i32]) -> bool {
    if report.len() < 2 {
        return true;
    }

    let mut increasing = true;
    let mut decreasing = true;

    for window in report.windows(2) {
        let diff = window[1] - window[0];
        if diff < -3 || diff > 3 || diff == 0 {
            return false;
        }
        if diff < 0 {
            increasing = false;
        }
        if diff > 0 {
            decreasing = false;
        }
    }

    increasing || decreasing
}

fn is_safe_with_dampener(report: &[i32]) -> bool {
    if is_safe(report) {
        return true;
    }

    for i in 0..report.len() {
        let mut modified = report.to_vec();
        modified.remove(i);
        if is_safe(&modified) {
            return true;
        }
    }

    false
}

fn validate_lines(lines: &[Vec<i32>]) -> usize {
    lines
        .iter()
        .filter(|&line| is_safe_with_dampener(line))
        .count()
}

fn main() {
    let input = match std::fs::read_to_string("input.txt") {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading input file: {}", e);
            return;
        }
    };

    let lines: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|word| word.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    let safe_count = validate_lines(&lines);
    println!("Safe lines with dampener: {}", safe_count);
}
