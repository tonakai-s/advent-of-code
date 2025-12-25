use std::time::Instant;

use crate::common;

pub fn part1() {
    let input = common::read_inp(6);
    let start = Instant::now();

    let duration = start.elapsed();
    let secs = duration.as_secs_f64();

    let mut problems_grid: Vec<Vec<u64>> = Vec::with_capacity(input.lines().count()-1);
    let mut operators: Vec<&str> = vec![];

    let mut lines = input.lines().peekable();
    while let Some(l) = lines.next() {
        if lines.peek().is_none() {
            operators = l.split_ascii_whitespace().map(str::trim).collect();
            continue;
        }
        let row: Vec<u64> = l.split_ascii_whitespace().map(|n| n.trim().parse().expect(&format!("n: {n}"))).collect();
        problems_grid.push(row);
    }

    let mut total = 0;
    for i in 0..problems_grid[0].len() {
        let operation = match operators[i] {
            "*" => u64::wrapping_mul,
            _ => u64::wrapping_add
        };
        let res = problems_grid.iter().skip(1).fold(problems_grid[0][i], |acc, r| operation(acc, r[i]));
        total += res;
    }
    
    println!("6/1: {total} - Elapsed time: {secs}");
}
