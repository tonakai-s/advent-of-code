use std::{fs::File, io::Read};

use day_1::solve;

fn main() {
    let path = "./input.txt";
    let mut file = File::open(&path).expect("Unable to open the file.");
    let mut buff = String::new();

    file.read_to_string(&mut buff).expect("Unable to read file content");

    solve::problem_1(buff.clone());
    solve::problem_2(buff);
}