use std::collections::HashSet;
use std::fs::read_to_string;

pub mod test;
use test::test::day6_p2;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

type Grid = Vec<Vec<char>>;

const OBSTACLE: char = '#';

fn string_to_grid(input: &str) -> Grid {
    input.lines().map(|line| line.chars().collect()).collect()
}

/// Finds the guard's starting position and direction.
fn find_guard(grid: &Grid) -> ((usize, usize), Direction) {
    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            let direction = match cell {
                '^' => Some(Direction::Up),
                '>' => Some(Direction::Right),
                'v' => Some(Direction::Down),
                '<' => Some(Direction::Left),
                _ => None,
            };
            if let Some(d) = direction {
                return ((i, j), d);
            }
        }
    }
    panic!("No guard found in the input");
}

/// Checks if a given position is within the grid.
fn in_bounds(grid: &Grid, x: isize, y: isize) -> bool {
    x >= 0 && y >= 0 && (x as usize) < grid.len() && (y as usize) < grid[0].len()
}

/// Returns true if there's an obstacle at the given position.
fn is_obstacle(grid: &Grid, x: isize, y: isize) -> bool {
    if !in_bounds(grid, x, y) {
        // If out-of-bounds, it's not an obstacle: the guard will leave the grid.
        return false;
    }
    grid[x as usize][y as usize] == OBSTACLE
}

fn turn_right(dir: Direction) -> Direction {
    match dir {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}

fn main() {
    let input = read_to_string("input.txt").expect("Failed to read input.txt");
    let grid = string_to_grid(&input);

    let ((mut x, mut y), mut dir) = find_guard(&grid);

    let mut visited_positions = HashSet::new();
    visited_positions.insert((x, y));

    // To prevent infinite loops, track visited states (position + direction)
    let mut seen_states = HashSet::new();
    seen_states.insert((x, y, dir));

    loop {
        let (next_x, next_y) = match dir {
            Direction::Up => (x as isize - 1, y as isize),
            Direction::Right => (x as isize, y as isize + 1),
            Direction::Down => (x as isize + 1, y as isize),
            Direction::Left => (x as isize, y as isize - 1),
        };

        // If next step is out-of-bounds, guard leaves the area
        if !in_bounds(&grid, next_x, next_y) {
            // Guard leaves the mapped area
            break;
        }

        // If there's an obstacle, turn right
        if is_obstacle(&grid, next_x, next_y) {
            dir = turn_right(dir);
        } else {
            // Move forward
            x = next_x as usize;
            y = next_y as usize;
            visited_positions.insert((x, y));
        }

        // Check for repeated state to prevent infinite loops
        if !seen_states.insert((x, y, dir)) {
            eprintln!("Detected a repeating state: infinite loop prevented.");
            break;
        }
    }

    println!(
        "Number of distinct visited positions: {}",
        visited_positions.len()
    );
    println!("{:?}", day6_p2());
}
