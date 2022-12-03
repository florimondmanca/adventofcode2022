use std::error::Error;
use std::fs;
use std::path::Path;

fn get_input() -> Result<String, Box<dyn Error>> {
    let path = Path::new("inputs/1.txt");
    let content = fs::read_to_string(path)?;
    return Ok(content);
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let content = get_input()?;

    let mut max_calories = 0;
    let mut calories = 0;

    for line in content.lines() {
        if line.is_empty() {
            if calories > max_calories {
                max_calories = calories;
            }
            calories = 0;
            continue;
        }

        calories += line.parse::<i32>().unwrap();
    }

    println!("Answer: {}", max_calories);

    Ok(())
}
