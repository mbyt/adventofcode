use std::fs;
use std::path::Path;

fn main() {
    let path = Path::new("input");
    assert!(path.exists());

    let contents = fs::read_to_string(path).unwrap();
    // TODO: more robust alternative, which also splits correlty at \r etc.
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

    // part 2

    let mut group_calories_sorted = group_calories.clone();
    group_calories_sorted.sort();
    group_calories_sorted.reverse();

    let top3_calories_sum: i32 = group_calories_sorted.iter().take(3).sum();

    println!("Sum of top 3 calories: {top3_calories_sum}");
}
