use std::collections::{HashMap, VecDeque};
use std::fs;

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
struct Point {
    x: usize,
    y: usize,
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let (h, w) = (map.len(), map[0].len());

    let mut start = Point { x: 0, y: 0 };
    let mut end = Point { x: 0, y: 0 };
    for (r, row) in map.iter().enumerate() {
        for (c, &ch) in row.iter().enumerate() {
            if ch == 'S' {
                start = Point { x: r, y: c };
            } else if ch == 'E' {
                end = Point { x: r, y: c };
            }
        }
    }

    let is_track = |r: usize, c: usize| {
        if r >= h || c >= w {
            false
        } else {
            let ch = map[r][c];
            ch == '.' || ch == 'S' || ch == 'E'
        }
    };

    let dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    let mut dist_no_cheat_from_S = vec![vec!(-1; w); h];
    {
        let mut q = VecDeque::new();
        dist_no_cheat_from_S[start.x][start.y] = 0;
        q.push_back(start);
        while let Some(p) = q.pop_front() {
            let d = dist_no_cheat_from_S[p.x][p.y];
            for &(dx, dy) in &dirs {
                let nx = p.x.wrapping_add(dx as usize);
                let ny = p.y.wrapping_add(dy as usize);
                if nx < h && ny < w && is_track(nx, ny) && dist_no_cheat_from_S[nx][ny] == -1 {
                    dist_no_cheat_from_S[nx][ny] = d + 1;
                    q.push_back(Point { x: nx, y: ny });
                }
            }
        }
    }

    let T_no_cheat = dist_no_cheat_from_S[end.x][end.y];
    if T_no_cheat == -1 {
        println!("No path to end even without cheating.");
        return;
    }
    let T_no_cheat = T_no_cheat as i32;

    let mut dist_no_cheat_to_E = vec![vec!(-1; w); h];
    {
        let mut q = VecDeque::new();
        dist_no_cheat_to_E[end.x][end.y] = 0;
        q.push_back(end);
        while let Some(p) = q.pop_front() {
            let d = dist_no_cheat_to_E[p.x][p.y];
            for &(dx, dy) in &dirs {
                let nx = p.x.wrapping_add(dx as usize);
                let ny = p.y.wrapping_add(dy as usize);
                if nx < h && ny < w && is_track(nx, ny) && dist_no_cheat_to_E[nx][ny] == -1 {
                    dist_no_cheat_to_E[nx][ny] = d + 1;
                    q.push_back(Point { x: nx, y: ny });
                }
            }
        }
    }

    let max_cheat_steps = 20;
    let mut seen_cheats = HashMap::new();
    let mut count_at_least_100 = 0;

    let mut visited_states = Vec::new();

    for x in 0..h {
        for y in 0..w {
            if dist_no_cheat_from_S[x][y] == -1 {
                continue;
            }

            let start_dist = dist_no_cheat_from_S[x][y];
            visited_states.clear();
            visited_states.resize(h * w * (max_cheat_steps + 1), false);

            let idx = |xx: usize, yy: usize, s: usize| {
                xx * (w * (max_cheat_steps + 1)) + yy * (max_cheat_steps + 1) + s
            };
            let mut q = VecDeque::new();

            q.push_back((x, y, 0));
            visited_states[idx(x, y, 0)] = true;

            while let Some((cx, cy, steps_used)) = q.pop_front() {
                if steps_used > 0 && steps_used <= max_cheat_steps && is_track(cx, cy) {
                    let after_dist = dist_no_cheat_to_E[cx][cy];
                    if after_dist != -1 {
                        let T_with_cheat = start_dist + steps_used as i32 + after_dist;
                        let time_saved = T_no_cheat - T_with_cheat;
                        if time_saved >= 100 {
                            let key = ((x, y), (cx, cy));
                            if !seen_cheats.contains_key(&key) {
                                seen_cheats.insert(key, time_saved);
                                count_at_least_100 += 1;
                            }
                        }
                    }
                }

                if steps_used < max_cheat_steps {
                    for &(dx, dy) in &dirs {
                        let nx = cx.wrapping_add(dx as usize);
                        let ny = cy.wrapping_add(dy as usize);
                        if nx < h && ny < w {
                            // During cheat steps, we can pass through walls
                            // No check for is_track needed here
                            let ns = steps_used + 1;
                            let idx2 = idx(nx, ny, ns);
                            if !visited_states[idx2] {
                                visited_states[idx2] = true;
                                q.push_back((nx, ny, ns));
                            }
                        }
                    }
                }
            }
        }
    }

    println!("{}", count_at_least_100);
}
