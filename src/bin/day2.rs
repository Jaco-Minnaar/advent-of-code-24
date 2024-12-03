use std::io::BufRead;

use advent_of_code::get_input;

pub fn main() {
    let input = get_input(2);

    let mut safe_count = 0;
    for line in input.lines() {
        let line = line.expect("Cannot get line");

        let levels: Vec<u32> = line.split(" ").map(|n| n.parse().unwrap()).collect();

        if is_safe(&levels) {
            safe_count += 1;
            continue;
        } else {
            for skip_index in 0..levels.len() {
                let mut levels = levels.clone();
                levels.remove(skip_index);

                if is_safe(&levels) {
                    safe_count += 1;
                    break;
                }
            }
        }
    }
    println!("{safe_count}");
}

fn is_safe(levels: &[u32]) -> bool {
    let mut asc = None;

    for i in 0..levels.len() - 1 {
        let l1 = levels[i];
        let l2 = levels[i + 1];

        asc = match l1 {
            l if l < l2 => Some(true),
            l if l > l2 => Some(false),
            _ => return false,
        };
    }

    let asc = asc.unwrap();

    for i in 0..levels.len() - 1 {
        let l1 = levels[i];
        let l2 = levels[i + 1];

        if is_problem(l1, l2, asc) {
            return false;
        }
    }

    true
}

fn is_problem(l1: u32, l2: u32, asc: bool) -> bool {
    if l1 == l2 {
        return false;
    }

    if asc {
        if l1 > l2 {
            return true;
        }
    } else if l1 < l2 {
        return true;
    }

    let diff = l1.abs_diff(l2);

    if !(1..=3).contains(&diff) {
        return true;
    }

    false
}
