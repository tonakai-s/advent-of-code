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

pub fn part2() {
    let input = common::read_ex_inp(3);

    let start = Instant::now();

    let mut total_joltage: u64 = 0;
    let mut added = 0;

    for bank in input.lines() {
        let mut joltage: u64 = 0;
        let mut idx = 0;
        let mut available_b = 12;
        let bank: Vec<u8> = bank.chars().map(|c| c.to_digit(10).unwrap() as u8).collect();

        for (i, n) in bank[0..bank.len()-available_b].iter().enumerate() {
            if *n > joltage as u8 {
                joltage = *n as u64;
                idx = i+1;
                if *n == 9 {
                    break;
                }
            }
        }
        available_b -= 1;
        joltage = joltage * (10u64.pow(available_b as u32)) as u64;

        //while idx < bank.len()-1-available_b && available_b > 1 {
        while idx < bank.len()-available_b && available_b > 1 {
            let mut j = 0;
            let mut ij = 0;
            for (i, n) in bank[idx..=(bank.len()-available_b)].iter().enumerate() {
                if *n > j {
                    j = *n;
                    ij = i;
                    if *n == 9 {
                        break;
                    }
                }
            }
            available_b -= 1;
            joltage += j as u64 * (10u64.pow(available_b as u32)) as u64;
            idx += ij+1;
        }
        if available_b > 1 {
            // 168736754411460
            // 168738209663590 *
            //joltage += bank[bank.len()-available_b..].iter().fold(0u64, |j, n| {
            joltage += bank[idx..].iter().fold(0u64, |j, n| {
                available_b -= 1;
                j + *n as u64 * (10u64.pow(available_b as u32))
            });
        } else {
            joltage += *bank[idx..].iter().max().unwrap() as u64;
        }
        println!("bank: {:?}", bank);
        println!("joltage: {joltage}");
        added += 1;
        total_joltage += joltage;
    }

    let duration = start.elapsed();
    let secs = duration.as_secs_f64();

    println!("added: {added}");
    println!("3/2: {total_joltage} - Elapsed time: {secs}");
}
