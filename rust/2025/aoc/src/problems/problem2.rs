use crate::common;

pub fn part1() {
    // let input = common::read_ex_inp(2);
    let input = common::read_inp(2);

    let mut res: usize = 0;
    input.split(",").for_each(|r| {
        let (start, end) = r.split_once("-").unwrap();
        println!("{start} - {end}");
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
    
    println!("Part 1: {res}");
}
