pub mod solve {
    pub fn problem_1(input: String) {
        let mut left: Vec<usize> = Vec::new();
        let mut right: Vec<usize> = Vec::new();

        input.lines().for_each(|line| {
            let mut nums = line.split_whitespace();
            left.push(nums.next().unwrap().parse().unwrap());
            right.push(nums.next().unwrap().parse().unwrap());
        });

        left.sort();
        right.sort();

        let mut accum: usize = 0;
        for i in 0..=left.len()-1 {
            if right[i] > left[i] {
                accum += right[i] - left[i];
            } else {
                accum += left[i] - right[i];
            }
        }

        println!("Problem 1: {:?}", accum);
    }

    pub fn problem_2(input: String) {
        let mut left: Vec<usize> = Vec::new();
        let mut right: Vec<usize> = Vec::new();

        input.lines().for_each(|line| {
            let mut nums = line.split_whitespace();
            left.push(nums.next().unwrap().parse().unwrap());
            right.push(nums.next().unwrap().parse().unwrap());
        });

        // TODO: Sort and check the sequence when hit the first occurrence, is how much faster?
        let mut accum: usize = 0;
        for n in left.iter() {
            accum += n * right.iter().filter(|inner_n| *inner_n == n).count()
        }

        println!("Problem 2: {:?}", accum);
    }
}