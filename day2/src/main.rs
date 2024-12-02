use std::error::Error;
use std::fs::File;
use std::io::Read;

fn main() -> Result<(), Box<dyn Error>> {
    // Read the input from the file
    let filename = "input.txt";
    let mut contents = String::new();
    File::open(filename)?.read_to_string(&mut contents)?;

    let safe_reports_count = contents
        .lines()
        .filter(|line| is_safe_with_dampener(line))
        .count();

    println!("Number of safe reports: {}", safe_reports_count);

    Ok(())
}

fn is_safe_with_dampener(report: &str) -> bool {
    let levels: Vec<i32> = report
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    if is_safe_report(&levels) {
        return true;
    }

    for i in 0..levels.len() {
        let mut modified_levels = levels.clone();
        modified_levels.remove(i); // Remove one level
        if is_safe_report(&modified_levels) {
            return true;
        }
    }

    false
}

fn is_safe_report(levels: &[i32]) -> bool {
    let mut is_increasing = true;
    let mut is_decreasing = true;

    for window in levels.windows(2) {
        let diff = window[1] - window[0];
        match diff {
             x if x.abs() < 1 || x.abs() > 3 => return false,
             x if x < 0 => is_increasing = false,
             x if x > 0 => is_decreasing = false,
             _ => {},
        }
    }

    is_increasing || is_decreasing
}
