use std::time::Instant;

use crate::common;

pub fn part1() {
    let input = common::read_inp(7);
    let start = Instant::now();

    let mut rows = input.lines();

    let mut beam_tracker: Vec<usize> = vec![];
    beam_tracker.push(rows.next().unwrap().find(|c: char| c == 'S').unwrap());
    
    let mut splitters = 0;
    while let Some(r) = rows.next() {
        let row: Vec<char> = r.chars().collect();
        for ti in (0..beam_tracker.len()).rev() {
            let t = beam_tracker[ti];
            if row[t] == '^' {
                splitters += 1;

                beam_tracker.remove(ti);
                if !beam_tracker.iter().any(|bt| *bt == t-1) {
                    beam_tracker.push(t-1);
                }


                if !beam_tracker.iter().any(|bt| *bt == t+1) {
                    beam_tracker.push(t+1);
                }
            }
        }
    }

    let duration = start.elapsed();
    let secs = duration.as_secs_f64();
    
    println!("7/1: {splitters} - Elapsed time: {secs}");
}
