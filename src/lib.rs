use std::{
    env,
    fs::File,
    io::{BufRead, BufReader, Seek, Write},
};

use reqwest::blocking::Client;

pub fn get_test_input(day: u32) -> impl BufRead {
    let file = File::open(format!("input/input{}.test", day)).expect("Could not open file");

    BufReader::new(file)
}

pub fn get_input(day: u32) -> impl BufRead {
    dotenvy::dotenv().unwrap();

    let path = format!("input/input{}", day);
    if let Ok(file) = File::open(&path) {
        eprintln!("Reading input from file");
        BufReader::new(file)
    } else {
        eprintln!("Fetching input from site");
        let client = Client::new();
        let res = client
            .get(format!("https://adventofcode.com/2024/day/{}/input", day))
            .header("Cookie", env::var("COOKIE").unwrap())
            .send()
            .expect("Could not get input");

        let content = res.bytes().expect("Could not convert response to text");

        let mut file = File::options()
            .create(true)
            .truncate(true)
            .read(true)
            .write(true)
            .open(path)
            .unwrap();
        file.write_all(&content).unwrap();

        file.rewind().unwrap();

        BufReader::new(file)
    }
}
