#![feature(iter_array_chunks)]

use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

fn main() {
    let char_to_priority_map = get_char_to_priority_map();

    // test
    let rucksacks = r#"
        vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw
    "#
    .trim();
    let ref_scores = vec![16, 38, 42, 22, 20, 19];

    rucksacks
        .lines()
        .into_iter()
        .zip(ref_scores.iter())
        .for_each(|(line, ref_score)| {
            let score = get_score_rucksack(line.trim(), &char_to_priority_map);
            assert_eq!(score, *ref_score);
        });

    let path = Path::new("input");
    assert!(path.exists());

    let contents = fs::read_to_string(path).unwrap().trim().to_owned();

    let mut total_score = 0;
    for line in contents.lines() {
        let score = get_score_rucksack(line, &char_to_priority_map);
        total_score += score;
    }

    println!("Total score = {total_score}");
    // regression test for refactoring
    assert_eq!(total_score, 8252);

    // part 2

    // test
    assert_eq!(
        sticker_attachment_effort(rucksacks, &char_to_priority_map),
        70
    );

    let priorities_sum = sticker_attachment_effort(&contents, &char_to_priority_map);
    println!("Priorities sum = {priorities_sum}");
    assert_eq!(priorities_sum, 2828);
}

fn sticker_attachment_effort(rucksacks: &str, char_to_priority_map: &HashMap<char, i32>) -> i32 {
    let mut priorities_sum = 0;
    assert_eq!(rucksacks.len() % 3, 0);
    for [line1, line2, line3] in rucksacks.lines().map(|line| line.trim()).array_chunks() {
        let ovlp_a = get_item_in_both_slices(line1.chars().collect(), line2.chars().collect());
        let ovlp_b = get_item_in_both_slices(line2.chars().collect(), line3.chars().collect());
        let ovlp = get_item_in_both_slices(ovlp_a, ovlp_b);
        //dbg!(&ovlp);
        assert_eq!(ovlp.len(), 1);
        let priority = char_to_priority_map[&ovlp[0]];
        //dbg!(priority);
        priorities_sum += priority;
    }
    //dbg!(priorities_sum);
    priorities_sum
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

    [('a', 1), ('z', 26), ('A', 27), ('Z', 52)]
        .into_iter()
        .for_each(|(chr, priority)| {
            assert_eq!(char_to_priority[&chr], priority);
        });
    //dbg!(&char_to_priority);
    char_to_priority
}

fn get_item_in_both_compartments(line: &str) -> char {
    let len = line.len();
    //dbg!(N);
    assert_eq!(len & 0b1, 0b0, "Error: even length required");
    let char_vec: Vec<char> = line.chars().into_iter().collect();
    //dbg!(&char_vec);
    let slice1 = char_vec[..len / 2].to_vec();
    let slice2 = char_vec[len / 2..].to_vec();
    let intersec_vec = get_item_in_both_slices(slice1, slice2);
    assert_eq!(intersec_vec.len(), 1);
    let item_in_both_compartments = intersec_vec[0];
    item_in_both_compartments
}

fn get_item_in_both_slices(slice1: Vec<char>, slice2: Vec<char>) -> Vec<char> {
    let comp1: HashSet<char> = slice1.iter().map(|c| c.clone()).collect();
    let comp2: HashSet<char> = slice2.iter().map(|c| c.clone()).collect();
    //dbg!(&comp1);
    //dbg!(&comp2);
    let intersec = comp1.intersection(&comp2);
    //dbg!(&intersec);
    // TODO: avoid building a vector
    let intersec_vec: Vec<char> = intersec.map(|c| c.clone()).collect();
    intersec_vec
}
