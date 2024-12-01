use std::error::Error;
use std::io::Read;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");
    let filename = "input.txt";
    let mut contents = vec![];
    File::open(filename)?.read_to_end(&mut contents)?;

    let file_content = String::from_utf8(contents)?;

    //split the file content into lines
    let mut first_half = Vec::new();
    let mut second_half = Vec::new();

    for line in file_content.lines() {
        //split each line into parts by whitespace
        let mut parts = line.split_whitespace();
        if let (Some(first), Some(second)) = (parts.next(), parts.next()) {
            first_half.push(first.parse::<i32>()?);
            second_half.push(second.parse::<i32>()?);
        }
    }

    //print results
    println!("First half: {:?}", first_half);
    println!("Second half: {:?}", second_half);
    println!("first half len: {}", first_half.len());
    println!("second half len: {}", second_half.len());

    first_half.sort();
    second_half.sort();
    let mut sum = 0;
    let mut similarity = 0;
    for (a, b) in first_half.iter().zip(second_half.iter()) {
        sum +=(a-b).abs();
        similarity += second_half.iter().filter(|&&x| x == *a).count() as i32*a;
    }
    println!("total sum is {}", sum);
    println!("total similarity is {}", similarity);
    Ok(())
}
