use std::time::Instant;

use crate::common;

pub fn part1() {
    let input = common::read_ex_inp(5);
    let start = Instant::now();
    
    let mut ranges: Vec<(u64, u64)> = vec![];

    let mut ingredients_db = input.lines();
    let mut fresh_ingredients = 0;

    loop {
        let line = ingredients_db.next().unwrap();
        
        if line.is_empty() {
            break;
        }
        println!("line: {line}");

        let (a, b) = line.split_once('-').unwrap();
        let mut r: (u64, u64) = (a.parse().unwrap(), b.parse().unwrap());

        if ranges.is_empty() {
            ranges.push(r);
            continue;
        }
        
        if ranges.iter().any(|(l, h)| *l <= r.0 && *h >= r.1) {
            continue;
        }

        if let Some(m) = ranges.iter().min_by(|a, b| a.0.cmp(&b.0)) && m.0 > r.0 {
            if m.1 > r.1 {
                ranges.push(r);
                continue;
            } else {
                ranges.push((r.0, m.0-1));
            }
        }
        if let Some(m) = ranges.iter().max_by(|a, b| a.0.cmp(&b.0)) && m.1 < r.1 {
            if m.0 < r.0 {
                ranges.push(r);
                continue;
            } else {
                ranges.push((m.0+1, r.1));
            }
        }
    }

    println!("ranges: {:?}", ranges);

    while let Some(ingredient_id) = ingredients_db.next() {
        let id: u64 = ingredient_id.parse().unwrap();
        //println!("id: {id}");

        if ranges.iter().filter(|r| r.0 <= id).any(|r| r.1 >= id) {
            fresh_ingredients += 1;
        }
    }

    let duration = start.elapsed();
    let secs = duration.as_secs_f64();
    
    println!("5/1: {fresh_ingredients} - Elapsed time: {secs}");
}
