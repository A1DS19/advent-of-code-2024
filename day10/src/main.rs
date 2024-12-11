use std::collections::{HashSet, VecDeque};

fn solve(input: &[String]) -> usize {
    let map: Vec<Vec<u8>> = input
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect();

    let rows = map.len();
    let cols = map[0].len();
    let mut total_score = 0;

    // Find all trailheads (height 0)
    for r in 0..rows {
        for c in 0..cols {
            if map[r][c] == 0 {
                let mut visited = HashSet::new();
                let mut queue = VecDeque::new();
                queue.push_back((r, c));
                let mut reachable_nines = HashSet::new();

                while let Some((x, y)) = queue.pop_front() {
                    let current_height = map[x][y];

                    if current_height == 9 {
                        reachable_nines.insert((x, y));
                        continue;
                    }

                    for (nx, ny) in neighbors(x, y, rows, cols) {
                        if map[nx][ny] == current_height + 1 && !visited.contains(&(nx, ny)) {
                            visited.insert((nx, ny));
                            queue.push_back((nx, ny));
                        }
                    }
                }

                total_score += reachable_nines.len();
            }
        }
    }

    total_score
}

fn neighbors(r: usize, c: usize, max_r: usize, max_c: usize) -> Vec<(usize, usize)> {
    let mut neigh = Vec::new();
    if r > 0 {
        neigh.push((r - 1, c));
    }
    if c > 0 {
        neigh.push((r, c - 1));
    }
    if r + 1 < max_r {
        neigh.push((r + 1, c));
    }
    if c + 1 < max_c {
        neigh.push((r, c + 1));
    }
    neigh
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let lines: Vec<String> = input.lines().map(String::from).collect();
    let result = solve(&lines);
    let result2 = solve_part2(&lines);

    println!("{}", result);
    println!("{}", result2);
}

use std::collections::HashMap;

fn solve_part2(input: &[String]) -> usize {
    let map: Vec<Vec<u8>> = input
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect();

    let rows = map.len();
    let cols = map[0].len();
    let mut total_rating = 0;

    for r in 0..rows {
        for c in 0..cols {
            if map[r][c] == 0 {
                let mut cache = HashMap::new();
                let rating = count_paths(&map, r, c, &mut cache);
                total_rating += rating;
            }
        }
    }

    total_rating
}

fn count_paths(
    map: &[Vec<u8>],
    r: usize,
    c: usize,
    cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if map[r][c] == 9 {
        return 1;
    }

    if let Some(&count) = cache.get(&(r, c)) {
        return count;
    }

    let current_height = map[r][c];
    let rows = map.len();
    let cols = map[0].len();
    let mut paths = 0;

    // Check all four directions
    for (dr, dc) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
        let new_r = r as isize + dr;
        let new_c = c as isize + dc;

        if new_r >= 0 && new_r < rows as isize && new_c >= 0 && new_c < cols as isize {
            let new_r = new_r as usize;
            let new_c = new_c as usize;

            if map[new_r][new_c] == current_height + 1 {
                paths += count_paths(map, new_r, new_c, cache);
            }
        }
    }

    cache.insert((r, c), paths);
    paths
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = vec![
            "89010123".to_string(),
            "78121874".to_string(),
            "87430965".to_string(),
            "96549874".to_string(),
            "45678903".to_string(),
            "32019012".to_string(),
            "01329801".to_string(),
            "10456732".to_string(),
        ];
        assert_eq!(solve(&input), 36);
    }
    #[test]
    fn test_example_part2() {
        let input = vec![
            "89010123".to_string(),
            "78121874".to_string(),
            "87430965".to_string(),
            "96549874".to_string(),
            "45678903".to_string(),
            "32019012".to_string(),
            "01329801".to_string(),
            "10456732".to_string(),
        ];
        assert_eq!(solve_part2(&input), 81);
    }
}
