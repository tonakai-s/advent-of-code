use crate::common;

pub fn part1() {
    let input = common::read_ex_inp(2);

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
    
    println!("2/1: {res}");
}

pub fn part2() {
    let input = common::read_inp(2);

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
            // TODO: must have a more performatic way to check this
            for chunk_size in 2..=(nstr.len()/2) {
                let chunk = nstr.get(0..chunk_size).unwrap().to_string();
                if chunk.repeat(nstr.len()/chunk_size) != nstr {
                    continue;
                }
                res += n;
                break;
            }
        }
    });
    
    println!("2/2: {res}");
}
