use std::fs;
use std::path::Path;

fn main() {
    let path = Path::new("input");
    assert!(path.exists());

    let contents = fs::read_to_string(path).unwrap();
    // safer alternative, as it will also replace \r etc.
    //let _split_white: Vec<&str> = contents.split_whitespace().collect();
    //let contents_cleaned = _split_white.join("\n");
    //let parts: Vec<&str> = contents_cleaned.split("\n\n").collect();
    let parts: Vec<&str> = contents.split("\n\n").collect();
    let n_parts = parts.len();

    let calories: Vec<Vec<i32>> = parts
        .iter()
        .map(|part| -> Vec<i32> {
            part.split_whitespace()
                .map(|line| line.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    let group_calories: Vec<i32> = calories
        .iter()
        .map(|values| values.iter().sum())
        .collect();

    let max_calories = *group_calories.iter().max().unwrap();

    println!("Calories, groups={n_parts}, max={max_calories}");
}
