use std::fs::read_to_string;

fn count_overlapping_substrings(s: &str, sub: &str) -> usize {
    if sub.is_empty() {
        return 0;
    }
    let mut count = 0;
    let mut pos = 0;
    while pos + sub.len() <= s.len() {
        if &s[pos..pos + sub.len()] == sub {
            count += 1;
        }
        pos += 1; // Move one character forward to allow overlapping matches
    }
    count
}

fn parse_horizontal(input: &str, word: &str) -> usize {
    input
        .lines()
        .map(|line| {
            count_overlapping_substrings(line, word)
                + count_overlapping_substrings(&line.chars().rev().collect::<String>(), word)
        })
        .sum()
}

fn parse_vertical(input: &str, word: &str) -> usize {
    let lines: Vec<&str> = input.lines().collect();
    let num_rows = lines.len();
    let num_cols = lines[0].len();
    let mut total = 0;

    for col in 0..num_cols {
        let column_string: String = lines
            .iter()
            .map(|line| line.chars().nth(col).unwrap())
            .collect();

        total += count_overlapping_substrings(&column_string, word);
        let reversed_column: String = column_string.chars().rev().collect();
        total += count_overlapping_substrings(&reversed_column, word);
    }

    total
}

fn parse_diagonals(input: &str, word: &str) -> usize {
    let lines: Vec<&str> = input.lines().collect();
    let num_rows = lines.len();
    let num_cols = lines[0].len();
    let mut total = 0;

    // Diagonals from top-left to bottom-right (down-right)
    for k in 0..(num_rows + num_cols - 1) {
        let mut diagonal = String::new();

        let row_start = if k >= num_cols - 1 {
            k - (num_cols - 1)
        } else {
            0
        };
        let row_end = k.min(num_rows - 1);

        for i in row_start..=row_end {
            let row = i;
            let col = k - i;
            diagonal.push(lines[row].chars().nth(col).unwrap());
        }

        if !diagonal.is_empty() {
            total += count_overlapping_substrings(&diagonal, word);
            let reversed_diagonal: String = diagonal.chars().rev().collect();
            total += count_overlapping_substrings(&reversed_diagonal, word);
        }
    }

    // Diagonals from top-right to bottom-left (down-left)
    for k in 0..(num_rows + num_cols - 1) {
        let mut diagonal = String::new();

        let row_start = if k >= num_cols - 1 {
            k - (num_cols - 1)
        } else {
            0
        };
        let row_end = k.min(num_rows - 1);

        for i in row_start..=row_end {
            let row = i;
            let col = (num_cols - 1) - (k - i);
            diagonal.push(lines[row].chars().nth(col).unwrap());
        }

        if !diagonal.is_empty() {
            total += count_overlapping_substrings(&diagonal, word);
            let reversed_diagonal: String = diagonal.chars().rev().collect();
            total += count_overlapping_substrings(&reversed_diagonal, word);
        }
    }

    total
}

fn count_x_mas(grid: &[Vec<char>]) -> usize {
    let num_rows = grid.len();
    let num_cols = grid[0].len();
    let mut total = 0;

    // Directions for diagonals: [(-1, -1), (1, 1)] and [(-1, 1), (1, -1)]
    for i in 0..num_rows {
        for j in 0..num_cols {
            if grid[i][j] == 'A' {
                // Check primary diagonal (top-left to bottom-right)
                let mut primary_matches = 0;
                if i > 0 && i + 1 < num_rows && j > 0 && j + 1 < num_cols {
                    let top_left = grid[i - 1][j - 1];
                    let bottom_right = grid[i + 1][j + 1];

                    if (top_left == 'M' && bottom_right == 'S')
                        || (top_left == 'S' && bottom_right == 'M')
                    {
                        primary_matches = 1;
                    }
                }

                // Check secondary diagonal (top-right to bottom-left)
                let mut secondary_matches = 0;
                if i > 0 && i + 1 < num_rows && j + 1 < num_cols && j > 0 {
                    let top_right = grid[i - 1][j + 1];
                    let bottom_left = grid[i + 1][j - 1];

                    if (top_right == 'M' && bottom_left == 'S')
                        || (top_right == 'S' && bottom_left == 'M')
                    {
                        secondary_matches = 1;
                    }
                }

                // If both diagonals form valid "MAS" or "SAM", increment total
                total += primary_matches * secondary_matches;
            }
        }
    }

    total
}

fn main() {
    let input = read_to_string("input.txt").expect("Failed to read input.txt");
    let word = "XMAS";

    // let horizontal = parse_horizontal(&input, word);
    // let vertical = parse_vertical(&input, word);
    // let diagonal = parse_diagonals(&input, word);

    // let total = horizontal + vertical + diagonal;

    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let total = count_x_mas(&grid);
    println!("Total: {}", total);
}
