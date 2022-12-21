// puzzle: https://adventofcode.com/2022/day/21

use std::collections::{HashSet, HashMap};
use std::{fs, path::Path};

#[derive(Debug, Clone)]
enum Operation {
    Addition(String, String),
    Subtraction(String, String),
    Multiplication(String, String),
    Division(String, String),
    Number(i64),
}

#[derive(Debug, Clone)]
struct Line {
    name: String,
    operation: Operation,
}

const ROOT_NAME: &str = "root";

fn main() {
    let example = get_example();

    let target_number = get_root_number(example);
    assert_eq!(target_number, Some(152));

    let path = Path::new("input");
    assert!(path.exists());
    let contents = fs::read_to_string(path).unwrap();
    let target_number = get_root_number(contents);

    println!("root number = {:?}", target_number);
}

fn get_root_number(example: String) -> Option<i64> {
    let operations: Vec<Line> = example.lines().map(|raw_line| {
        let line = raw_line.trim();
        let parts: Vec<&str> = line.split(":").collect();
        assert_eq!(parts.len(), 2);
        let name = parts[0].trim();
        let tokens: Vec<&str> = parts[1].trim().split_whitespace().collect();
        let operation = match tokens[..] {
            [name1, "+", name2] => Operation::Addition(name1.to_owned(), name2.to_owned()),
            [name1, "-", name2] => Operation::Subtraction(name1.to_owned(), name2.to_owned()),
            [name1, "*", name2] => Operation::Multiplication(name1.to_owned(), name2.to_owned()),
            [name1, "/", name2] => Operation::Division(name1.to_owned(), name2.to_owned()),
            [num] => Operation::Number(num.parse::<i64>().unwrap()),
            _ => { panic!("Unknown operation 1 {:?}", tokens); }
        };
        Line { name: name.to_owned(), operation }
    }).collect();
    // cross check that for each there is only a single operation
    let names_set: HashSet<String> = operations
        .iter()
        .map(|Line { name, operation: _ }| name.clone()).collect();
    assert_eq!(names_set.len(), operations.len());
    let mut resolved: HashMap<String, i64> = HashMap::new();
    let mut open: HashMap<String, Operation> = HashMap::new();
    for op in &operations {
        match op {
            Line {name, operation: Operation::Number(num)} => {
                resolved.insert(name.clone(), *num);
            }
            Line {name, operation} => {
                open.insert(name.clone(), operation.clone());
            }
        }
    }
    assert_eq!(resolved.len() + open.len(), operations.len());
    let mut target_number: Option<i64> = None;
    while open.len() > 0 {
        let mut to_remove: Vec<String> = Vec::new();
        for (name, operation) in &open {
            match operation {
                Operation::Addition(name1, name2 ) => {
                    if resolved.contains_key(name1) && resolved.contains_key(name2) {
                        resolved.insert(
                            name.to_owned(), resolved[name1] + resolved[name2]
                        );
                        to_remove.push(name.to_owned());
                    }
                }
                Operation::Subtraction(name1, name2 ) => {
                    if resolved.contains_key(name1) && resolved.contains_key(name2) {
                        resolved.insert(
                            name.to_owned(), resolved[name1] - resolved[name2]
                        );
                        to_remove.push(name.to_owned());
                    }
                }
                Operation::Multiplication(name1, name2 ) => {
                    if resolved.contains_key(name1) && resolved.contains_key(name2) {
                        resolved.insert(
                            name.to_owned(), resolved[name1] * resolved[name2]
                        );
                        to_remove.push(name.to_owned());
                    }
                }
                Operation::Division(name1, name2 ) => {
                    if resolved.contains_key(name1) && resolved.contains_key(name2) {
                        resolved.insert(
                            name.to_owned(), resolved[name1] / resolved[name2]
                        );
                        to_remove.push(name.to_owned());
                    }
                }
                _ => { panic!("Unknown operation 2 {:?}", operation); }
            }
        }
        if to_remove.contains(&ROOT_NAME.to_owned()) {
            target_number = Some(resolved[ROOT_NAME.clone()]);
            println!("break here break");
            break;
        }

        for name in &to_remove {
            open.remove(name);
        }
    }
    target_number
}

fn get_example() -> String {
    r#"
        root: pppw + sjmn
        dbpl: 5
        cczh: sllz + lgvd
        zczc: 2
        ptdq: humn - dvpt
        dvpt: 3
        lfqf: 4
        humn: 5
        ljgn: 2
        sjmn: drzm * dbpl
        sllz: 4
        pppw: cczh / lfqf
        lgvd: ljgn * ptdq
        drzm: hmdt - zczc
        hmdt: 32
    "#.trim().to_owned()
}
