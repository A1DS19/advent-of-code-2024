use std::collections::{HashSet, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point(usize, usize);

struct Region {
    area: usize,
    sides: HashSet<(i32, i32)>,
}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Failed to read input");
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let total = process_map(&map);
    println!("Total price: {}", total);
}

fn process_map(map: &[Vec<char>]) -> usize {
    let mut visited = HashSet::new();
    let mut total = 0;

    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if visited.contains(&Point(i, j)) {
                continue;
            }

            let region = find_region(map, i, j, &mut visited);
            total += region.area * region.sides.len();
        }
    }
    total
}

fn find_region(
    map: &[Vec<char>],
    start_i: usize,
    start_j: usize,
    visited: &mut HashSet<Point>,
) -> Region {
    let mut queue = VecDeque::new();
    let mut region = Region {
        area: 0,
        sides: HashSet::new(),
    };

    let plot_type = map[start_i][start_j];
    queue.push_back(Point(start_i, start_j));

    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    while let Some(Point(i, j)) = queue.pop_front() {
        if !visited.insert(Point(i, j)) {
            continue;
        }

        region.area += 1;

        for &(di, dj) in &directions {
            let ni = i as i32 + di;
            let nj = j as i32 + dj;

            if ni >= 0 && nj >= 0 && ni < map.len() as i32 && nj < map[0].len() as i32 {
                let ni = ni as usize;
                let nj = nj as usize;

                if map[ni][nj] == plot_type {
                    queue.push_back(Point(ni, nj));
                } else {
                    region.sides.insert((di, dj));
                }
            } else {
                region.sides.insert((di, dj));
            }
        }
    }

    region
}
