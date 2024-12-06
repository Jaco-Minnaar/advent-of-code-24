use std::collections::HashMap;
use std::io::BufRead;

use advent_of_code::get_input;

fn main() {
    let input = get_input(5);

    let lines = input.lines().filter_map(|line| {
        if let Ok(line) = line {
            Some(line)
        } else {
            None
        }
    });

    //let correct = count(lines);
    let correct = sort_and_sum(lines);

    println!("{correct}");
}

fn count(mut lines: impl Iterator<Item = impl AsRef<str>>) -> usize {
    let rule_lines = lines
        .by_ref()
        .take_while(|line| !line.as_ref().trim().is_empty());
    let rules = parse_rules(rule_lines);

    sum_valid_middles(lines, &rules)
}

fn sort_and_sum(mut lines: impl Iterator<Item = impl AsRef<str>>) -> usize {
    let rule_lines = lines
        .by_ref()
        .take_while(|line| !line.as_ref().trim().is_empty());
    let rules = parse_rules(rule_lines);

    sort_and_sum_invalids(lines, &rules)
}

fn parse_rules(lines: impl Iterator<Item = impl AsRef<str>>) -> HashMap<usize, Vec<usize>> {
    let mut rules = HashMap::new();
    for line in lines {
        let line = line.as_ref();
        let mut parts = line.split('|');

        let num1 = parts.next().unwrap().parse::<usize>().unwrap();
        let num2 = parts.next().unwrap().parse::<usize>().unwrap();

        let before = rules.entry(num2).or_insert(Vec::new());
        before.push(num1);
    }

    rules
}

fn sum_valid_middles(
    update_lines: impl Iterator<Item = impl AsRef<str>>,
    rules: &HashMap<usize, Vec<usize>>,
) -> usize {
    let mut sum = 0;
    for line in update_lines {
        let line = line.as_ref();
        let nums: Vec<usize> = line
            .split(',')
            .map(|n| n.parse::<usize>().unwrap())
            .collect();

        if is_valid(&nums, rules) {
            let middle_idx = nums.len() / 2;
            sum += nums[middle_idx];
        }
    }

    sum
}

fn sort_and_sum_invalids(
    update_lines: impl Iterator<Item = impl AsRef<str>>,
    rules: &HashMap<usize, Vec<usize>>,
) -> usize {
    let mut sum = 0;

    for (line_no, line) in update_lines.enumerate() {
        let line = line.as_ref();
        let mut nums: Vec<usize> = line
            .split(',')
            .map(|n| n.parse::<usize>().unwrap())
            .collect();

        println!("Line: {line_no}");
        let mut i = 0;
        let len = nums.len();
        let mut was_invalid = false;
        while i < len {
            let num = nums[i];
            let Some(before) = rules.get(&num) else {
                i += 1;
                continue;
            };

            if let Some((j, other_num)) = nums
                .iter()
                .enumerate()
                .skip(i + 1)
                .rev()
                .find(|(_, other_num)| iter_has(before.iter(), other_num))
            {
                println!("Error: {num} is before {other_num}");

                let r_num = nums.remove(i);
                assert_eq!(num, r_num);
                println!("Moving {num} (idx {i}) to idx {j}");
                nums.insert(j, num);
                was_invalid = true;
            } else {
                i += 1;
            }
        }

        assert!(is_valid(&nums, rules));

        if was_invalid {
            let middle_idx = nums.len() / 2;
            sum += nums[middle_idx];
        }
    }

    sum
}

fn is_valid(nums: &[usize], rules: &HashMap<usize, Vec<usize>>) -> bool {
    for (i, num) in nums.iter().enumerate() {
        let Some(before) = rules.get(num) else {
            continue;
        };

        if nums
            .iter()
            .skip(i + 1)
            .any(|other_num| iter_has(before.iter(), other_num))
        {
            return false;
        }
    }

    true
}

fn iter_has<T>(mut iter: impl Iterator<Item = T>, needle: T) -> bool
where
    T: PartialEq,
{
    iter.any(|n| n == needle)
}

#[cfg(test)]
mod test {
    use crate::{count, sort_and_sum};

    #[test]
    fn count_valid_updates_test_input() {
        let input = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;

        let valid = count(input.lines());

        assert_eq!(valid, 143);
    }

    #[test]
    fn sort_and_sum_test_input() {
        let input = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;

        let sum = sort_and_sum(input.lines());

        assert_eq!(sum, 123);
    }

    #[test]
    fn sort_and_sum_simple() {
        let input = r#"47|53
97|13
75|13
53|13

97,13,75,53,47"#;
        let sum = sort_and_sum(input.lines());

        assert_eq!(sum, 47);
    }
}
