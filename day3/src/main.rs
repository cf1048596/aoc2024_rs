use std::error::Error;
use regex::Regex;
use std::fs::File;
use std::io::Read;

fn main() -> Result<(), Box<dyn Error>> {
    // Read the input from the file
    let filename = "input.txt";
    let mut contents = String::new();
    File::open(filename)?.read_to_string(&mut contents)?;

    let total_sum: i32 = contents
        .lines()
        .map(|line| check_line(line))
        .sum();

    println!("total sum {}", total_sum);

    Ok(())
}

fn check_line(line: &str) -> i32{
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    re.captures_iter(line)
        .map(|captures| {
            let x: i32 = captures[1].parse().unwrap();
            let y: i32 = captures[2].parse().unwrap();
            x * y
        })
        .sum()
}
