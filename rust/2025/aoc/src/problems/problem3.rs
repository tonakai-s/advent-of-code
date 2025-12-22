use std::time::Instant;

use crate::common;

pub fn part1() {
    let input = common::read_inp(3);

    let start = Instant::now();

    let mut joltage: u32 = 0;

    for bank in input.lines() {

        let bank: Vec<u8> = bank.chars().map(|c| c.to_digit(10).unwrap() as u8).collect();
        let mut dec: (u8, usize) = (0, 0);
        for (i, n) in bank[0..bank.len()-1].iter().enumerate() {
            if *n > dec.0 {
                dec = (*n, i);
            }
        }
        let un = bank[((dec.1)+1)..].iter().max().unwrap();

        joltage += (dec.0 * 10 + un) as u32;
    }

    let duration = start.elapsed();
    let secs = duration.as_secs_f64();
    
    println!("3/1: {joltage} - Elapsed time: {secs}");
}
