use std::fs;
use std::io::{self, Read};

const INPUT_FILE: &str = "input.txt";

trait Sum {
    fn sum(&self) -> i32;
}

impl Sum for Vec<i32> {
    fn sum(&self) -> i32 {
        self.iter().sum()
    }
}

trait ParseI32 {
    fn parse_i32(&self) -> i32;
}

impl ParseI32 for &str {
    fn parse_i32(&self) -> i32 {
        self.parse::<i32>().unwrap()
    }
}

fn load_input() -> io::Result<String> {
    let mut file = fs::File::open(INPUT_FILE)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn get_line_vector(index: usize, lines: &Vec<Vec<&str>>) -> Vec<i32> {
    lines
        .iter()
        .map(|line| line[index].parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let lines: Vec<Vec<&str>> = input
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<&str>>())
        .collect();

    let mut left = get_line_vector(0, &lines);

    let mut right = get_line_vector(1, &lines);

    left.sort();
    right.sort();

    (left, right)
}

fn similarity_score(left: Vec<i32>, right: Vec<i32>) -> i32 {
    left.iter()
        .map(|l| {
            let count = right.iter().filter(|&&r| r == *l).count() as i32;
            l * count
        })
        .sum()
}

fn main() {
    let input = match load_input() {
        Ok(input) => input,
        Err(e) => {
            eprintln!("Error loading input: {}", e);
            return;
        }
    };

    let (list_0, list_1) = parse_input(&input);

    let total_distance: i32 = list_0
        .iter()
        .zip(list_1.iter())
        .map(|(a, b)| (a - b).abs())
        .sum();

    println!("{}", total_distance);

    let score = similarity_score(list_0, list_1);

    println!("{}", score);
}
