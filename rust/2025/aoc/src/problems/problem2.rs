use std::time::Instant;

use crate::common;

pub fn part1() {
    let input = common::read_ex_inp(2);

    let start = Instant::now();

    let mut res: usize = 0;
    input.split(",").for_each(|r| {
        let (start, end) = r.split_once("-").unwrap();
        for n in start.trim().parse::<usize>().unwrap()..=end.trim().parse::<usize>().unwrap() {
            let nstr = n.to_string();
            if nstr.len() % 2 == 1 {
                continue;
            }
            let (l, r) = nstr.split_at(nstr.len()/2);
            if l == r {
                res += n;
            }
        }
    });

    let duration = start.elapsed();
    let secs = duration.as_secs_f64();
    
    println!("2/1: {res} - Elapsed time: {secs}");
}

pub fn part2() {
    let input = common::read_ex_inp(2);

    let start = Instant::now();

    let mut res: usize = 0;
    input.split(",").for_each(|r| {
        let (start, end) = r.split_once("-").unwrap();
        for n in start.parse::<usize>().unwrap()..=end.trim().parse::<usize>().unwrap() {
            let nstr = n.to_string();

            if nstr.len() < 2 {
                continue;
            }
            if {
                let fc = nstr.chars().next().unwrap();
                nstr.chars().all(|c| c == fc)
            } {
                res += n;
                continue;
            }

            'chunk: for chunk_size in 2..=(nstr.len()/2) {
                let chunk = &nstr[0..chunk_size];

                let mut chunk_start = chunk_size;
                loop {
                    if &nstr[chunk_start..chunk_start+chunk_size] != chunk {
                        continue 'chunk;
                    }
                    
                    chunk_start = chunk_start + chunk_size;
                    if chunk_start == nstr.len() {
                        res += n;
                        break 'chunk;
                    }
                    if chunk_start + chunk_size > nstr.len() {
                        continue 'chunk;
                    }
                }
            }
        }
    });
    
    let duration = start.elapsed();
    let secs = duration.as_secs_f64();
    
    println!("2/2: {res} - Elapsed time: {secs}");
}
