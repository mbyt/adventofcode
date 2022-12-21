use serde_json::Value;
use std::{fs, path::Path};

const IDX_OFFSET: usize = 1;

pub fn main() {
    // test
    let example = get_example();
    let indices_of_in_order_pairs = get_indices_of_in_order_pairs(&example);
    let index_sum: usize = indices_of_in_order_pairs
        .iter()
        .fold(0, |acc, idx| acc + (idx + IDX_OFFSET));
    assert_eq!(index_sum, 13);

    let debug = r#"
        [[]]
        [[[]]]
    "#
    .trim()
    .to_owned();
    dbg!(get_indices_of_in_order_pairs(&debug));

    // main part
    let path = Path::new("src/day13/input");
    assert!(path.exists());
    let contents = fs::read_to_string(path).unwrap();
    let indices_of_in_order_pairs = get_indices_of_in_order_pairs(contents.trim());
    dbg!(&indices_of_in_order_pairs);
    let index_sum: usize = indices_of_in_order_pairs
        .iter()
        .fold(0, |acc, idx| acc + (idx + IDX_OFFSET));

    println!("Sum of in order indices {index_sum}");
}

fn get_example() -> String {
    r#"
        [1,1,3,1,1]
        [1,1,5,1,1]

        [[1],[2,3,4]]
        [[1],4]

        [9]
        [[8,7,6]]

        [[4,4],4,4]
        [[4,4],4,4,4]

        [7,7,7,7]
        [7,7,7]

        []
        [3]

        [[[]]]
        [[]]

        [1,[2,[3,[4,[5,6,7]]]],8,9]
        [1,[2,[3,[4,[5,6,0]]]],8,9]
    "#
    .trim()
    .to_owned()
}

fn get_indices_of_in_order_pairs(example: &str) -> Vec<usize> {
    let parts: Vec<&str> = example.split("\n\n").collect();
    dbg!(parts.len());
    let mut indices: Vec<usize> = Vec::new();
    for (i, part) in parts.iter().enumerate() {
        let pair: Vec<&str> = part.split_whitespace().map(|s| s.trim()).collect();
        assert_eq!(pair.len(), 2);
        let left_raw: Value = serde_json::from_str(pair[0]).unwrap();
        let right_raw: Value = serde_json::from_str(pair[1]).unwrap();

        let Value::Array(left) = left_raw else {
            panic!("Error cannot parse {left_raw} as json");
        };
        let Value::Array(right) = right_raw else {
            panic!("Error cannot parse {right_raw} as json");
        };
        let in_order = compare_arrays(&left, &right);
        if in_order {
            indices.push(i);
        }
    }
    indices
}

fn compare_arrays(left: &Vec<Value>, right: &Vec<Value>) -> bool {
    if right.len() < left.len() {
        return false;
    }
    // zip rust doc: it will first try to advance the first iterator at most one
    // time and if it still yielded an item try to advance the second iterator
    // at most one time.
    for (left_el, right_el) in left.iter().zip(right.iter()) {
        let mut in_order = true;
        match (left_el, right_el) {
            // If both values are integers, the lower integer should come first
            (Value::Number(ref left), Value::Number(ref right)) => {
                let left_i64 = left.as_i64().unwrap();
                let right_i64 = right.as_i64().unwrap();
                if left_i64 > right_i64 {
                    in_order = false;
                }
            }
            // If exactly one value is an integer, convert the integer to a list
            // which contains that integer as its only value, then retry the
            // comparison
            (Value::Array(left), Value::Number(_)) if left.is_empty() => {
                in_order = true;
            }
            (Value::Number(_), Value::Array(right)) if right.is_empty() => {
                in_order = false;
            }
            (Value::Array(left), Value::Number(right_num)) => {
                in_order = compare_arrays(
                    &vec![left[0].clone()],
                    &vec![Value::Number(right_num.clone())],
                );
            }
            (Value::Number(left_num), Value::Array(right)) => {
                in_order = compare_arrays(
                    &vec![Value::Number(left_num.clone())],
                    &vec![right[0].clone()],
                );
            }
            (Value::Array(left), Value::Array(right)) if left.is_empty() && right.is_empty() => {
                in_order = true;
            }
            // If both values are lists, compare the first value of each list, then the second
            // value, and so on. If the left list runs out of items first, the inputs are in
            // the right order. If the right list runs out of items first, the inputs are not
            // in the right order.
            (Value::Array(left), Value::Array(right)) => {
                in_order = compare_arrays(left, right);
            }
            _ => {
                panic!(
                    "Not implemented yet, left_el={:?}, right_el={:?}",
                    left_el, right_el
                );
            }
        }
        if !in_order {
            return false;
        }
    }
    true
}
