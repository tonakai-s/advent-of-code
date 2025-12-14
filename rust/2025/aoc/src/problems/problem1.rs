use crate::common;

const DIAL_SIZE: u16 = 100;
pub fn part1() {
    let input = common::read_ex_inp(1);
    let mut points: u64 = 0;
    let mut dial: isize = 50;
    for line in input.lines() {
        let mut num: isize = line[1..].parse().unwrap();
        if &line[0..1] == "L" {
            num = -num;
        }
        dial = ( dial + num ).rem_euclid(DIAL_SIZE as isize);
        if dial == 0 {
            points += 1;
        }
    }

    println!("1/1: {points}");
}
