use std::fs;
use std::path::Path;
use std::collections::HashSet;

fn main() {
    let line = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    let pos = fun_name(line);
    assert_eq!(pos, 7);
    // main
    let path = Path::new("input");
    assert!(path.exists());
    
    let contents = fs::read_to_string(path).unwrap().trim().to_owned();
    let pos = fun_name(&contents);

    println!("pos: {pos}");
    // regression test for refactoring
    assert_eq!(pos, 1757);
}

fn fun_name(line: &str) -> usize {
    let n = 4;
    let cvec: Vec<char> = line.chars().collect();
    let mut pos = 0;
    for i in 0..(cvec.len() - n) {
        let cset: HashSet<char> = (&cvec[i..i+n]).iter().cloned().collect();
        if cset.len() == n {
            pos = i + n;
            break;
        }
    }
    pos
}
