use std::time::Instant;

use crate::common;

pub fn part1() {
    let input = common::read_ex_inp(6);
    let start = Instant::now();

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
        total += problems_grid.iter().skip(1).fold(problems_grid[0][i], |acc, r| operation(acc, r[i]));
    }

    let duration = start.elapsed();
    let secs = duration.as_secs_f64();
    
    println!("6/1: {total} - Elapsed time: {secs}");
}

pub fn part2() {
    let input = common::read_ex_inp(6);
    let start = Instant::now();

    let mut problems_positions: Vec<(usize, usize)> = vec![];
    let mut lines: Vec<&str> = input.lines().collect();

    lines.last().unwrap().chars().skip(1).enumerate().fold((0usize, 0usize), |tracker, (i, c)| {
        if c.is_whitespace() {
            tracker
        } else {
            problems_positions.push((tracker.0, i-1));
            (i+1, i+1)
        }
    });
    problems_positions.push((problems_positions[problems_positions.len()-1].1+2, lines.last().unwrap().len()-1));

    let operators: Vec<&str> = lines.pop().unwrap().split_ascii_whitespace().collect();

    let lines = lines.iter().map(|l| l.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

    let mut problems_grid: Vec<Vec<u64>> = Vec::with_capacity(input.lines().count()-1);

    for (sr, er) in &problems_positions {
        let mut problem_row: Vec<u64> = vec![];
        for p in *sr..=*er {
            let mut numstr = String::new();
            for line in &lines {
                if line[p].is_digit(10) {
                    numstr.push(line[p]);
                }
            }
            problem_row.push(numstr.parse().unwrap());
        }
        problems_grid.push(problem_row);
    }

    let mut total = 0;
    for (i, set) in problems_grid.iter().enumerate() {
        let operation = match operators[i] {
            "*" => u64::wrapping_mul,
            _ => u64::wrapping_add
        };
        total += set.iter().skip(1).fold(set[0], |acc, p| operation(acc, *p));
    }

    let duration = start.elapsed();
    let secs = duration.as_secs_f64();
    
    println!("6/2: {total} - Elapsed time: {secs}");
}
