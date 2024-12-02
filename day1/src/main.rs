use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Read;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");
    let filename = "input.txt";
    let mut contents = vec![];
    File::open(filename)?.read_to_end(&mut contents)?;

    let file_content = String::from_utf8(contents)?;

    let mut first_half = Vec::new();
    let mut second_half = Vec::new();

    for line in file_content.lines() {
        let mut parts = line.split_whitespace();
        if let (Some(first), Some(second)) = (parts.next(), parts.next()) {
            first_half.push(first.parse::<i32>()?);
            second_half.push(second.parse::<i32>()?);
        }
    }

    println!("First half: {:?}", first_half);
    println!("Second half: {:?}", second_half);
    println!("first half len: {}", first_half.len());
    println!("second half len: {}", second_half.len());

    first_half.sort();
    second_half.sort();

    let mut freq_map: HashMap<i32, i32> = HashMap::new();
    for &num in &second_half {
        *freq_map.entry(num).or_insert(0) += 1;
    }

    let mut sum = 0;
    for (a, b) in first_half.iter().zip(second_half.iter()) {
        sum +=(a-b).abs();
    }


    let mut similarity = 0;
    for &a in &first_half {
        if let Some(&count) = freq_map.get(&a) {
            similarity += count * a;
        }
    }

    println!("total sum is {}", sum);
    println!("total similarity is {}", similarity);
    Ok(())
}
