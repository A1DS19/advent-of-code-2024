use std::collections::{HashMap, HashSet};
use std::fs;

/*
garden plot = single type of plant and is indicated by a single letter on your map
region = When multiple garden plots are growing the same type of plant and are touching (horizontally or vertically)

AAAA
BBCD
BBCC
EEEC

area = the number of garden plots in the region
perimeter = Visually indicating the sides of plots in each region that contribute to the perimeter using - and |, the above map's regions' perimeters are measured as follows:
+-+-+-+-+
|A A A A|
+-+-+-+-+     +-+
              |D|
+-+-+   +-+   +-+
|B B|   |C|
+   +   + +-+
|B B|   |C C|
+-+-+   +-+ +
          |C|
+-+-+-+   +-+
|E E E|
+-+-+-+
*/

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Edge {
    // An edge is represented by its starting and ending coordinates.
    // We'll store edges in a canonical form (start < end lexicographically) so that duplicates are avoided.
    start: (usize, usize),
    end: (usize, usize),
}

impl Edge {
    fn new(a: (usize, usize), b: (usize, usize)) -> Edge {
        if a < b {
            Edge { start: a, end: b }
        } else {
            Edge { start: b, end: a }
        }
    }

    fn direction(&self) -> (isize, isize) {
        // direction vector
        (
            self.end.0 as isize - self.start.0 as isize,
            self.end.1 as isize - self.start.1 as isize,
        )
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input.txt");
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let regions = calculate_regions(&map);

    let total_price: usize = regions.iter().map(|&(area, sides)| area * sides).sum();

    println!("Total price of fencing all regions: {}", total_price);
}

fn calculate_regions(map: &[Vec<char>]) -> Vec<(usize, usize)> {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut regions = Vec::new();

    let rows = map.len();
    let cols = map[0].len();

    for i in 0..rows {
        for j in 0..cols {
            if !visited.contains(&(i, j)) {
                let plot = map[i][j];
                // Explore region of same-type plots
                let mut stack = vec![(i, j)];
                let mut region_cells = Vec::new();
                let mut in_region = false;

                while let Some((x, y)) = stack.pop() {
                    if visited.contains(&(x, y)) {
                        continue;
                    }
                    if map[x][y] == plot {
                        visited.insert((x, y));
                        in_region = true;
                        region_cells.push((x, y));
                        for &(dx, dy) in &directions {
                            let nx = x as isize + dx;
                            let ny = y as isize + dy;
                            if nx >= 0 && ny >= 0 && (nx as usize) < rows && (ny as usize) < cols {
                                if map[nx as usize][ny as usize] == plot
                                    && !visited.contains(&(nx as usize, ny as usize))
                                {
                                    stack.push((nx as usize, ny as usize));
                                }
                            }
                        }
                    }
                }

                if in_region {
                    let area = region_cells.len();
                    let sides = count_sides(map, &region_cells);
                    regions.push((area, sides));
                }
            }
        }
    }

    regions
}

fn count_sides(map: &[Vec<char>], region_cells: &[(usize, usize)]) -> usize {
    // We first identify all boundary edges.
    // A boundary edge occurs where a cell in the region has a neighbor outside the region (or out of bounds).
    let region_set: HashSet<(usize, usize)> = region_cells.iter().copied().collect();
    let mut boundary_edges: HashSet<Edge> = HashSet::new();
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    let rows = map.len();
    let cols = map[0].len();

    for &(x, y) in region_cells {
        for &(dx, dy) in &directions {
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            if nx < 0
                || ny < 0
                || nx as usize >= rows
                || ny as usize >= cols
                || !region_set.contains(&(nx as usize, ny as usize))
            {
                // This is a boundary edge
                // Edge between (x,y) and the "outside" along one side of the cell
                // We represent this edge as the boundary between two points on the grid.
                // Edges run between grid corners. Let's map each cell edge to grid coordinates as if cell corners are at integer coordinates.
                // A cell (x,y) occupies coordinates in [x,x+1] x [y,y+1] space.
                // The top edge of (x,y) would be between (x,y) and (x,y+1) in terms of corner coordinates.
                // Actually, let's consider cell corners as (r,c) for top-left corners.

                // If dx=0 and dy=1 (right edge), edge is between (x,y+1) and (x+1,y+1)
                // If dx=0 and dy=-1 (left edge), edge is between (x,y) and (x+1,y)
                // If dx=1 and dy=0 (down edge), edge is between (x+1,y) and (x+1,y+1)
                // If dx=-1 and dy=0 (up edge), edge is between (x,y) and (x,y+1)

                let edge = match (dx, dy) {
                    (0, 1) => Edge::new((x, y + 1), (x + 1, y + 1)),
                    (0, -1) => Edge::new((x, y), (x + 1, y)),
                    (1, 0) => Edge::new((x + 1, y), (x + 1, y + 1)),
                    (-1, 0) => Edge::new((x, y), (x, y + 1)),
                    _ => unreachable!(),
                };
                boundary_edges.insert(edge);
            }
        }
    }

    // Now we have a set of edges that form one or more closed polygons.
    // We must:
    // 1) Group them into loops.
    // 2) For each loop, count how many straight segments (sides) it has.
    // Each edge is either vertical or horizontal.
    // We can reconstruct loops by following connected edges. Edges connect at their endpoints.

    let mut edges_by_start: HashMap<(usize, usize), Vec<Edge>> = HashMap::new();
    for e in &boundary_edges {
        edges_by_start.entry(e.start).or_default().push(*e);
        edges_by_start.entry(e.end).or_default().push(*e);
    }

    let mut used = HashSet::new();
    let mut total_sides = 0;

    for &edge in &boundary_edges {
        if used.contains(&edge) {
            continue;
        }
        // Follow edges to form a loop
        let loop_edges = form_loop(edge, &edges_by_start, &mut used);
        // Count sides in this loop:
        let sides = count_polygon_sides(&loop_edges);
        total_sides += sides;
    }

    total_sides
}

fn form_loop(
    start_edge: Edge,
    edges_by_start: &HashMap<(usize, usize), Vec<Edge>>,
    used: &mut HashSet<Edge>,
) -> Vec<Edge> {
    // We start from `start_edge`, and follow connections until we return to the start.
    let mut loop_edges = Vec::new();
    let mut current_edge = start_edge;
    let mut current_point = current_edge.end;
    used.insert(current_edge);
    loop_edges.push(current_edge);

    while current_point != start_edge.start {
        // Find the next edge that is connected to current_point and not used
        if let Some(edges) = edges_by_start.get(&current_point) {
            // Should find exactly one unused edge that continues the polygon
            let mut found = None;
            for &e in edges {
                if !used.contains(&e) && (e.start == current_point || e.end == current_point) {
                    found = Some(e);
                    break;
                }
            }
            if let Some(e) = found {
                // Continue the loop
                used.insert(e);
                loop_edges.push(e);
                // Move current_point to the next point in the chain
                if e.start == current_point {
                    current_point = e.end;
                } else {
                    current_point = e.start;
                }
            } else {
                // Should not happen if edges form a closed loop
                break;
            }
        } else {
            // No continuation, malformed boundary?
            break;
        }
    }

    loop_edges
}

fn count_polygon_sides(loop_edges: &[Edge]) -> usize {
    // Count how many straight segments the polygon has.
    // We do this by looking at the direction of each edge and counting direction changes.
    // Each edge is either horizontal (delta row =0) or vertical (delta col=0).
    // We'll represent direction as 'H' or 'V'.
    let mut directions = Vec::with_capacity(loop_edges.len());

    for e in loop_edges {
        let d = e.direction();
        // direction: horizontal if d.0 == 0, vertical if d.1 == 0
        if d.0 == 0 {
            directions.push('H');
        } else {
            directions.push('V');
        }
    }

    // Now count segments by merging consecutive edges with the same direction
    // The number of sides is the number of times direction changes when going around the loop.
    // Since it's a closed loop, we also must check the wrap-around.
    let mut sides = 1; // start with 1 segment
    for i in 1..directions.len() {
        if directions[i] != directions[i - 1] {
            sides += 1;
        }
    }
    // Check wrap-around:
    if directions[0] != directions[directions.len() - 1] {
        // If the first and last edges differ in direction, we actually have one more corner.
        // But wait, we started counting from sides=1. Actually, we counted every change.
        // The counting above already accounts for all changes between consecutive edges including last->first if we consider a loop.
        // We must consider the polygon is closed, so we have to ensure direction changes count the last->first transition as well.
        // In the logic above, we only counted i in [1..len), we must also check last->first explicitly:
        sides += 1;
    }

    sides
}
