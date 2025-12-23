use std::time::Instant;

use crate::common;

pub fn part1() {
    let input = common::read_inp(4);
    let start = Instant::now();
    
    let grid: Vec<Vec<char>> = input.lines()
        .map(|r| r.chars().collect())
        .collect();
    
    let mut removed_rolls = 0;
    for (i, l) in grid.iter().enumerate() {
        for (j, c) in l.iter().enumerate() {
            let mut neighbours = 0;
            if *c == '.' {
                continue;
            }
            if i > 0 {
                if j > 0 {
                    if grid[i-1][j-1] == '@' {
                        neighbours += 1;
                    }
                }
                if grid[i-1][j] == '@' {
                    neighbours += 1;
                }
                if j < l.len()-1 {
                    if grid[i-1][j+1] == '@' {
                        neighbours += 1;
                    }
                }
            }
            if j > 0 {
                if grid[i][j-1] == '@' {
                    neighbours += 1;
                }
            }
            if j < l.len()-1 {
                if grid[i][j+1] == '@' {
                    neighbours += 1;
                }
            }
            if i < grid.len()-1 {
                if j > 0 {
                    if grid[i+1][j-1] == '@' {
                        neighbours += 1;
                    }
                }
                if grid[i+1][j] == '@' {
                    neighbours += 1;
                }
                if j < l.len()-1 {
                    if grid[i+1][j+1] == '@' {
                        neighbours += 1;
                    }
                }
            }

            if neighbours < 4 {
                removed_rolls += 1;
            }
        }
    }

    let duration = start.elapsed();
    let secs = duration.as_secs_f64();
    
    println!("4/1: {removed_rolls} - Elapsed time: {secs}");
}

pub fn part2() {
    let input = common::read_ex_inp(4);
    let start = Instant::now();
    
    let mut grid: Vec<Vec<char>> = input.lines()
        .map(|r| r.chars().collect())
        .collect();
    
    let mut total = 0;
    loop {
        let mut remov_rolls: Vec<(usize, usize)> = vec![];
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                let mut neighbours = 0;
                if grid[i][j] == '.' {
                    continue;
                }
                if i > 0 {
                    if j > 0 {
                        if grid[i-1][j-1] == '@' {
                            neighbours += 1;
                        }
                    }
                    if grid[i-1][j] == '@' {
                        neighbours += 1;
                    }
                    if j < grid[i].len()-1 {
                        if grid[i-1][j+1] == '@' {
                            neighbours += 1;
                        }
                    }
                }
                if j > 0 {
                    if grid[i][j-1] == '@' {
                        neighbours += 1;
                    }
                }
                if j < grid[i].len()-1 {
                    if grid[i][j+1] == '@' {
                        neighbours += 1;
                    }
                }
                if i < grid.len()-1 {
                    if j > 0 {
                        if grid[i+1][j-1] == '@' {
                            neighbours += 1;
                        }
                    }
                    if grid[i+1][j] == '@' {
                        neighbours += 1;
                    }
                    if j < grid[i].len()-1 {
                        if grid[i+1][j+1] == '@' {
                            neighbours += 1;
                        }
                    }
                }

                if neighbours < 4 {
                    remov_rolls.push((i, j));
                }
            }
        }
        if remov_rolls.len() == 0 {
            break;
        }
        total += remov_rolls.len();
        remov_rolls.iter().for_each(|(i, j)| grid[*i][*j] = '.');
    }

    let duration = start.elapsed();
    let secs = duration.as_secs_f64();
    
    println!("4/2: {total} - Elapsed time: {secs}");
}
