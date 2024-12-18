use std::collections::VecDeque;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let lines = input.lines();

    let mut grid = vec![vec![false; 71]; 71]; // false means uncorrupted

    let positions: Vec<(usize, usize)> = lines
        .map(|line| {
            let parts: Vec<usize> = line
                .split(',')
                .map(|part| part.trim().parse::<usize>().unwrap_or(0))
                .collect();
            if parts.len() == 2 {
                (parts[0], parts[1])
            } else {
                (0, 0)
            }
        })
        .collect();

    for (x, y) in positions {
        if x <= 70 && y <= 70 {
            grid[y][x] = true; // Mark as corrupted

            // Check if there is still a path from (0,0) to (70,70)
            let mut visited = vec![vec![false; 71]; 71];
            let mut queue = VecDeque::new();
            if !grid[0][0] {
                queue.push_back((0usize, 0usize));
                visited[0][0] = true;
            }

            let directions = [(0i32, 1), (1, 0), (0, -1), (-1, 0)];

            let mut found = false;
            while let Some((cx, cy)) = queue.pop_front() {
                if cx == 70 && cy == 70 {
                    found = true;
                    break;
                }

                for (dx, dy) in &directions {
                    let nx = cx as i32 + dx;
                    let ny = cy as i32 + dy;

                    if nx >= 0 && nx <= 70 && ny >= 0 && ny <= 70 {
                        let nx = nx as usize;
                        let ny = ny as usize;
                        if !grid[ny][nx] && !visited[ny][nx] {
                            visited[ny][nx] = true;
                            queue.push_back((nx, ny));
                        }
                    }
                }
            }

            if !found {
                println!("{},{}", x, y);
                return;
            }
        }
    }

    println!("The exit is always reachable.");
}
