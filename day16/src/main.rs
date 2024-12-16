use std::{
    collections::{HashMap, HashSet},
    fs,
};

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
struct State {
    position: (usize, usize),
    direction: (i32, i32),
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input.txt");
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    // Find start and end positions
    let mut start_pos = (0, 0);
    let mut end_pos = (0, 0);
    for (i, row) in grid.iter().enumerate() {
        for (j, &ch) in row.iter().enumerate() {
            if ch == 'S' {
                start_pos = (i, j);
            }
            if ch == 'E' {
                end_pos = (i, j);
            }
        }
    }

    // Start facing east as per instructions
    let initial_state = State {
        position: start_pos,
        direction: (0, 1),
    };

    // Store costs for each state
    let mut costs: HashMap<State, i32> = HashMap::new();
    costs.insert(initial_state, 0);

    // States to explore
    let mut to_visit = vec![initial_state];

    // Possible rotations: left and right 90 degrees
    let rotations = [
        // Rotate left: (dx, dy) -> (-dy, dx)
        |d: (i32, i32)| (-d.1, d.0),
        // Rotate right: (dx, dy) -> (dy, -dx)
        |d: (i32, i32)| (d.1, -d.0),
    ];

    // Mark tiles that are part of any best path
    let mut best_path_tiles: HashSet<(usize, usize)> = HashSet::new();

    // Find the minimal cost to reach the end
    let mut min_end_cost = i32::MAX;

    while let Some(current) = to_visit.pop() {
        let current_cost = *costs.get(&current).unwrap();

        // Check if we reached the end
        if grid[current.position.0][current.position.1] == 'E' {
            min_end_cost = min_end_cost.min(current_cost);
            best_path_tiles.insert(current.position); // Add end position to best path
            continue;
        }

        // Try moving forward
        let new_pos = (
            (current.position.0 as i32 + current.direction.0) as usize,
            (current.position.1 as i32 + current.direction.1) as usize,
        );

        if new_pos.0 < grid.len() && new_pos.1 < grid[0].len() && grid[new_pos.0][new_pos.1] != '#'
        {
            let new_state = State {
                position: new_pos,
                direction: current.direction,
            };
            let new_cost = current_cost + 1;

            if !costs.contains_key(&new_state) || costs[&new_state] > new_cost {
                costs.insert(new_state, new_cost);
                to_visit.push(new_state);
            }

            // Mark this position as part of a possible best path
            if new_cost == min_end_cost {
                best_path_tiles.insert(new_pos);
            }
        }

        // Try rotating
        for rotation in rotations.iter() {
            let new_direction = rotation(current.direction);
            let new_state = State {
                position: current.position,
                direction: new_direction,
            };
            let new_cost = current_cost + 1000;

            if !costs.contains_key(&new_state) || costs[&new_state] > new_cost {
                costs.insert(new_state, new_cost);
                to_visit.push(new_state);
            }
        }
    }

    // Count the number of tiles in best_path_tiles
    println!(
        "Number of tiles that are part of at least one best path: {}",
        best_path_tiles.len()
    );
}
