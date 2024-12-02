use std::io::BufRead;

use advent_of_code::{get_input, get_test_input};

pub fn main() {
    let input = get_input(2);

    let mut safe_count = 0;
    let mut safe_lines = Vec::new();
    for (i, line) in input.lines().enumerate() {
        let line = line.expect("Cannot get line");

        let levels: Vec<u32> = line.split(" ").map(|n| n.parse().unwrap()).collect();

        let mut safe = true;
        let mut asc = None;

        for i in 0..levels.len() - 1 {
            let l1 = levels[i];
            let l2 = levels[i + 1];

            if l1 < l2 {
                asc = Some(true);
            } else if l1 > l2 {
                asc = Some(false);
            } else {
                safe = false;
            }
        }

        if safe {
            let asc = asc.unwrap();

            for i in 0..levels.len() - 1 {
                let l1 = levels[i];
                let l2 = levels[i + 1];

                if is_problem(l1, l2, asc) {
                    safe = false;
                    break;
                }
            }

            if safe {
                //dbg!(&levels);
                safe_count += 1;
                safe_lines.push((i, levels));
                continue;
            }
        }

        //for skip_index in 0..levels.len() {
        //    let mut levels = levels.clone();
        //    levels.remove(skip_index);
        //
        //    let mut safe = true;
        //    let mut asc = None;
        //    for i in 0..levels.len() - 1 {
        //        let l1 = levels[i];
        //        let l2 = levels[i + 1];
        //
        //        if is_problem(l1, l2, &mut asc) {
        //            safe = false;
        //            break;
        //        }
        //    }
        //
        //    if safe {
        //        //dbg!(&levels);
        //        safe_count += 1;
        //        safe_lines.push((i, levels));
        //        break;
        //    }
        //}
        //
        //for (line_no, levels) in &safe_lines {
        //    dbg!((line_no, levels));
        //}

        //    problem_idxs.dedup();
        //    if problem_idxs.is_empty() {
        //        safe_count += 1;
        //    } else {
        //        dbg!((&levels, &problem_idxs));
        //        for problem_i in problem_idxs.iter() {
        //            let mut safe = true;
        //            for i in 0..levels.len() - 1 {
        //                if i == *problem_i {
        //                    continue;
        //                }
        //                let l1 = levels[i];
        //                let l2 = if i + 1 == *problem_i {
        //                    if i + 2 >= levels.len() {
        //                        safe = false;
        //                        break;
        //                    }
        //                    levels[i + 2]
        //                } else {
        //                    levels[i + 1]
        //                };
        //
        //                if is_problem(l1, l2, &mut asc) {
        //                    safe = false;
        //                    break;
        //                }
        //            }
        //
        //            if safe {
        //                safe_count += 1;
        //                break;
        //            }
        //        }
        //    }
    }
    println!("{safe_count}");
    println!("{}", safe_lines.len());
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
