use std::cmp::min;
use std::fs::File;
use std::io::{self, BufRead};

const MAX: i32 = 9999;

fn read_matrix_from_file(file_path: &str) -> (Vec<Vec<i32>>, usize) {
    let mut matrix = Vec::new();
    
    if let Ok(file) = File::open(file_path) {
        for line in io::BufReader::new(file).lines() {
            if let Ok(content) = line {
                let row: Vec<i32> = content
                    .split_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect();
                matrix.push(row);
            }
        }
    }

    let n = matrix.len();
    (matrix, n)
}

fn tsp(mark: i32, position: usize, distan: &Vec<Vec<i32>>, dp: &mut Vec<Vec<i32>>) -> i32 {
    let n = distan.len();
    let completed_visit = (1 << n) - 1;
    if mark == completed_visit {
        return distan[position][0];
    }

    if dp[mark as usize][position] != -1 {
        return dp[mark as usize][position];
    }

    let mut answer = MAX;

    for city in 0..n {
        if (mark & (1 << city)) == 0 {
            let new_answer = distan[position][city] + tsp(mark | (1 << city), city, distan, dp);
            answer = min(answer, new_answer);
        }
    }

    dp[mark as usize][position] = answer;
    answer
}

fn main() {
    let mut input = String::new();

    // Prompt the user for input
    println!("Please enter file path:");

    // Read the input from stdin and handle any potential errors
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    // Trim the input to remove any trailing newline or whitespace characters
    let file_path = input.trim();

    let (distan, n) = read_matrix_from_file(file_path);

    let mut dp = vec![vec![-1; n]; 1 << n];

    println!("Minimum Distance Travelled -> {}", tsp(1, 0, &distan, &mut dp));
}
