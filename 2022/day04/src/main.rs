use std::collections::HashSet;
use std::fs;
use std::path::Path;

fn main() {
    let contents = r#"
        2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8
    "#
    .trim();

    assert_eq!(camp_cleanup_reconsideration_count(contents), 2);

    let path = Path::new("input");
    assert!(path.exists());

    let contents = fs::read_to_string(path).unwrap().trim().to_owned();
    let total_reconsideration_count = camp_cleanup_reconsideration_count(&contents);

    println!("Total reconsideration count: {total_reconsideration_count}");
    // regression test for refactoring
    assert_eq!(total_reconsideration_count, 483);
}

fn camp_cleanup_reconsideration_count(contents: &str) -> i32 {
    let mut total_score: i32 = 0;
    for line_wspace in contents.lines() {
        let line = line_wspace.trim();
        let parts: Vec<&str> = line.split(",").collect();
        assert_eq!(parts.len(), 2);
        // range_from_part
        let srng1 = set_from_str_range(parts[0]);
        let srng2 = set_from_str_range(parts[1]);
        if srng1.is_subset(&srng2) | srng2.is_subset(&srng1) {
            total_score += 1;
        }
    }
    total_score
}

fn set_from_str_range(line: &str) -> HashSet<i32> {
    let start_stop: Vec<i32> = line.split('-').map(|s| s.parse::<i32>().unwrap()).collect();
    assert_eq!(start_stop.len(), 2);
    let range: HashSet<i32> = (start_stop[0]..=start_stop[1]).collect();
    range
}
