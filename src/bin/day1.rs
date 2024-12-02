use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn main() {
    second();
}

fn first() {
    let file = File::open("input/input1").expect("Could not open file");

    let br = BufReader::new(file);

    let mut lhs = Vec::new();
    let mut rhs = Vec::new();

    for line in br.lines() {
        let line = line.expect("Could not get line");

        let mut nums = line.split("   ");
        let num1 = nums.next().unwrap().parse::<u32>().unwrap();
        let num2 = nums.next().unwrap().parse::<u32>().unwrap();

        lhs.push(num1);
        rhs.push(num2);
    }

    lhs.sort();
    rhs.sort();

    let mut sum = 0;
    for (num1, num2) in lhs.iter().zip(rhs) {
        let diff = num1.abs_diff(num2);

        sum += diff;
    }

    println!("{sum}");
}

fn second() {
    let file = File::open("input/input1").expect("Could not open file");

    let br = BufReader::new(file);

    let mut lhs = Vec::new();
    let mut rhs = Vec::new();

    for line in br.lines() {
        let line = line.expect("Could not get line");

        let mut nums = line.split("   ");
        let num1 = nums.next().unwrap().parse::<u32>().unwrap();
        let num2 = nums.next().unwrap().parse::<u32>().unwrap();

        lhs.push(num1);
        rhs.push(num2);
    }

    lhs.sort();
    rhs.sort();

    let mut nums = HashMap::new();
    let mut last_num = None;
    let mut num_n = 0;
    for num in rhs.iter() {
        let Some(last) = last_num else {
            last_num = Some(*num);
            num_n += 1;
            continue;
        };

        if *num == last {
            num_n += 1;
        } else {
            nums.insert(last, num_n);
            num_n = 1;
            last_num = Some(*num);
        }
    }

    nums.insert(last_num.unwrap(), num_n);

    let mut sum = 0;
    for num in lhs.iter() {
        let count = if let Some(num) = nums.get(num) {
            *num
        } else {
            0
        };

        sum += count * *num;
    }

    println!("{sum}");
}
