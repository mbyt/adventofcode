use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

fn main() {
    let char_to_priority_map = get_char_to_priority_map();

    let score = get_score_rucksack("vJrwpWtwJgWrhcsFMMfFFhFp", &char_to_priority_map);
    assert_eq!(score, 16);
    let score = get_score_rucksack("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL", &char_to_priority_map);
    assert_eq!(score, 38);
    let score = get_score_rucksack("PmmdzqPrVvPwwTWBwg", &char_to_priority_map);
    assert_eq!(score, 42);
    let score = get_score_rucksack("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn", &char_to_priority_map);
    assert_eq!(score, 22);
    let score = get_score_rucksack("ttgJtRGJQctTZtZT", &char_to_priority_map);
    assert_eq!(score, 20);
    let score = get_score_rucksack("CrZsJsPPZsGzwwsLwLmpwMDw", &char_to_priority_map);
    assert_eq!(score, 19);

    let path = Path::new("input");
    assert!(path.exists());

    let contents = fs::read_to_string(path).unwrap();

    let mut total_score = 0;
    for line in contents.lines() {
        let score = get_score_rucksack(line, &char_to_priority_map);
        total_score += score;
    }

    println!("Total score = {total_score}");
    // regression test for refactoring
    assert_eq!(total_score, 8252);
}

fn get_score_rucksack(line: &str, char_to_priority_map: &HashMap<char, i32>) -> i32 {
    let item_in_both_compartments = get_item_in_both_compartments(line);
    let score = char_to_priority_map[&item_in_both_compartments];
    score
}

fn get_char_to_priority_map() -> HashMap<char, i32> {
    // TODO: simplify without separte variable
    let alphabet_lower = 'a'..='z';
    let alphabet_upper = 'A'..='Z';
    //let char_to_priority_vec: HashMap<char, i32> = HashMap::from(alphabet_lower.into_iter()
    let _char_to_priority_vec: Vec<(char, i32)> = alphabet_lower
        .into_iter()
        .chain(alphabet_upper.into_iter())
        .enumerate()
        .map(|(i, c)| (c, (i + 1) as i32))
        .collect();
    //dbg!(&_char_to_priority_vec);
    let char_to_priority: HashMap<char, i32> = _char_to_priority_vec.into_iter().collect();
    //dbg!(&char_to_priority);
    assert_eq!(char_to_priority[&'a'], 1);
    assert_eq!(char_to_priority[&'z'], 26);
    assert_eq!(char_to_priority[&'A'], 27);
    assert_eq!(char_to_priority[&'Z'], 52);
    char_to_priority
}

fn get_item_in_both_compartments(line: &str) -> char {
    let len = line.len();
    //dbg!(N);
    assert_eq!(len & 0b1, 0b0, "Error: even length required");
    let char_vec: Vec<char> = line.chars().into_iter().collect();
    //dbg!(&char_vec);
    let comp1: HashSet<char> = char_vec[..len / 2].iter().map(|c| c.clone()).collect();
    let comp2: HashSet<char> = char_vec[len / 2..].iter().map(|c| c.clone()).collect();
    //dbg!(&comp1);
    //dbg!(&comp2);
    let intersec = comp1.intersection(&comp2);
    //dbg!(&intersec);
    // TODO: avoid building a vector
    let intersec_vec: Vec<char> = intersec.map(|c| c.clone()).collect();
    assert_eq!(intersec_vec.len(), 1);
    let item_in_both_compartments = intersec_vec[0];
    item_in_both_compartments
}
