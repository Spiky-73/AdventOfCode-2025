use std::io::BufRead;

pub fn solve_p1<F: BufRead>(input: F) {
    let mut fresh_ranges: Vec<(usize, usize)> = Vec::new();
    let mut fresh = 0;

    let mut done_ranges = false;
    for l in input.lines() {
        let line = l.unwrap();
        if line == "" {
            done_ranges = true;
        } else if !done_ranges {
            let nums = line.split('-').collect::<Vec<&str>>();
            let start = nums.iter().nth(0).unwrap().parse::<usize>().unwrap();
            let end = nums.iter().nth(1).unwrap().parse::<usize>().unwrap();
            fresh_ranges.push((start, end));
        } else {
            let num = line.parse::<usize>().unwrap();
            for &(f, t) in &fresh_ranges {
                if f <= num && num <= t {
                    fresh += 1;
                    break;
                }
            }
        }
    }
    println!("{}", fresh)
}

pub fn solve_p2<F: BufRead>(input: F) {
    let mut ranges: Vec<(usize, usize)> = Vec::new();
    let mut count = 0;
    for l in input.lines() {
        let line = l.unwrap();
        if line == "" {
            break;
        }
        let nums = line.split('-').collect::<Vec<&str>>();
        let start = nums.iter().nth(0).unwrap().parse::<usize>().unwrap();
        let end = nums.iter().nth(1).unwrap().parse::<usize>().unwrap() + 1;

        let mut i = ranges.len();
        for index in 0..ranges.len() {
            if start < ranges[index].0 {
                i = index;
                break;
            }
        }
        if i != 0 && ranges[i-1].0 <= start && end <= ranges[i-1].1 {
            continue;
        }
        if i == 0 || ranges[i - 1].1 < start {
            ranges.insert(i, (start, end));
        } else {
            i-=1;
            count -= ranges[i].1 - ranges[i].0;
            ranges[i].1 = end;
        }

        let mut j = i + 1;
        while j < ranges.len() && ranges[j].1 <= ranges[i].1 {
            count -= ranges[j].1 - ranges[j].0;
            j+=1;
        }
        for k in (i+1..j).rev() {
            ranges.remove(k);
        }

        if i != ranges.len()-1 && ranges[i+1].0 < ranges[i].1 {
            ranges[i].1 = ranges[i+1].1;
            count -= ranges[i+1].1 - ranges[i+1].0;
            ranges.remove(i+1);
        }

        count += ranges[i].1 - ranges[i].0;
    }
    println!("{}", count)
}
