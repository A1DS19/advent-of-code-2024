use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

/* Advent of Code - Day 6 - Surasia */

const PATH_TO_INPUT: &str = "input.txt";
const LINE_WIDTH: usize = 130;
const LINE_COUNT: usize = 130;

#[derive(PartialEq, Eq, Debug, Clone, Hash, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Bottom,
}

#[derive(PartialEq, Eq, Debug)]
struct GuardPosition(usize, usize, Direction);

fn figure_out_guard(pos: &mut GuardPosition, grid: &[Vec<char>]) -> Option<(usize, usize)> {
    let last_pos = &pos.2;
    let mut position = (0, 0);
    match last_pos {
        Direction::Up => {
            if pos.1 == 0 {
                return None;
            }
            if grid[pos.0][pos.1 - 1] == '#' {
                pos.2 = Direction::Right;
            } else {
                pos.1 -= 1;
                position = (pos.0, pos.1);
            }
        }
        Direction::Bottom => {
            if pos.1 == LINE_COUNT - 1 {
                return None;
            }
            if grid[pos.0][pos.1 + 1] == '#' {
                pos.2 = Direction::Left;
            } else {
                pos.1 += 1;
                position = (pos.0, pos.1);
            };
        }
        Direction::Left => {
            if pos.0 == 0 {
                return None;
            }
            if grid[pos.0 - 1][pos.1] == '#' {
                pos.2 = Direction::Up;
            } else {
                pos.0 -= 1;
                position = (pos.0, pos.1);
            }
        }
        Direction::Right => {
            if pos.0 == LINE_COUNT - 1 {
                return None;
            }
            if grid[pos.0 + 1][pos.1] == '#' {
                pos.2 = Direction::Bottom;
            } else {
                pos.0 += 1;
                position = (pos.0, pos.1);
            }
        }
    }
    Some(position)
}

fn create_grid(reader: BufReader<File>) -> (Vec<Vec<char>>, GuardPosition) {
    let mut grid = vec![vec![' '; LINE_WIDTH]; LINE_COUNT];
    let mut guard_position = GuardPosition(0, 0, Direction::Up);

    for (idx, line) in reader.lines().map_while(Result::ok).enumerate() {
        for c in line.char_indices() {
            match c.1 {
                '^' => guard_position = GuardPosition(c.0, idx, Direction::Up),
                '>' => guard_position = GuardPosition(c.0, idx, Direction::Right),
                '<' => guard_position = GuardPosition(c.0, idx, Direction::Left),
                'v' => guard_position = GuardPosition(c.0, idx, Direction::Bottom),
                _ => {}
            }
            grid[c.0][idx] = c.1;
        }
    }
    (grid, guard_position)
}

fn calculate_path(grid: &[Vec<char>], guard_position: GuardPosition) -> HashSet<(usize, usize)> {
    let mut all_positions = HashSet::new();
    let mut curr_pos = guard_position;

    all_positions.insert((curr_pos.0, curr_pos.1));
    while curr_pos.0 < LINE_COUNT && curr_pos.1 < LINE_COUNT {
        if let Some(pos) = figure_out_guard(&mut curr_pos, grid) {
            all_positions.insert(pos);
        } else {
            break;
        }
    }
    all_positions
}

pub fn day6_p1() {
    let file = File::open(PATH_TO_INPUT).expect("Failed to open file!");
    let br = BufReader::new(file);
    let (grid, guard_position) = create_grid(br);

    let path = calculate_path(&grid, guard_position);
    println!("Day 6 possible steps: {}", path.len() - 1);
}

pub fn day6_p2() {
    let file = File::open(PATH_TO_INPUT).expect("Failed to open file!");
    let br = BufReader::new(file);
    let (grid, guard_position) = create_grid(br);

    let start_pos = (guard_position.0, guard_position.1);
    let mut valid_positions = 0;

    for y in 0..LINE_COUNT {
        for x in 0..LINE_WIDTH {
            if (x, y) == start_pos || grid[x][y] == '#' {
                continue;
            }

            let mut test_grid = grid.clone();
            test_grid[x][y] = '#';

            let mut curr_pos = GuardPosition(guard_position.0, guard_position.1, guard_position.2);
            let mut visited = HashSet::new();
            visited.insert((curr_pos.0, curr_pos.1, curr_pos.2));

            loop {
                if figure_out_guard(&mut curr_pos, &test_grid).is_none() {
                    break;
                }
                let state = (curr_pos.0, curr_pos.1, curr_pos.2);
                if visited.contains(&state) {
                    valid_positions += 1;
                    break;
                }
                visited.insert(state);
            }
        }
    }

    println!("Day 6 possible obstructions: {valid_positions}");
}
