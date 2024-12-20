use std::{collections::HashSet, fs};

fn count_ways(design: &str, allowed_patterns: &HashSet<&str>) -> usize {
    let n = design.len();
    let chars = design.as_bytes();
    let mut dp = vec![0usize; n + 1];
    dp[0] = 1;

    for i in 1..=n {
        for pattern in allowed_patterns {
            let plen = pattern.len();
            if i >= plen && &chars[i - plen..i] == pattern.as_bytes() {
                dp[i] += dp[i - plen];
            }
        }
    }

    dp[n]
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read input file");
    let segments: Vec<&str> = input.split('\n').collect();

    let mut allowed_patterns: HashSet<&str> = HashSet::new();
    for p in segments[0].split(',') {
        allowed_patterns.insert(p.trim());
    }

    let mut line_index = 1;
    while line_index < segments.len() && segments[line_index].trim().is_empty() {
        line_index += 1;
    }

    let mut total_ways = 0usize;

    for i in line_index..segments.len() {
        let design = segments[i].trim();
        if !design.is_empty() {
            let ways = count_ways(design, &allowed_patterns);
            total_ways += ways;
        }
    }

    println!("{}", total_ways);
}
