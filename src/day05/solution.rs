use std::collections::{HashSet, VecDeque};
use std::fs;
use std::path::Path;
#[derive(Debug)]
struct Move {
    quantity: usize,
    from_stack_idx: usize,
    to_stack_idx: usize,
}

pub fn main() {
    let ref_stacks: Vec<VecDeque<char>> = vec![
        vec!['N', 'Z'].into_iter().collect(),
        vec!['D', 'C', 'M'].into_iter().collect(),
        vec!['P'].into_iter().collect(),
    ];

    // test
    let example = get_example();
    let (moves, n_stacks, mut stacks) = get_stacks_from_str(example);
    assert_eq!(stacks, ref_stacks);
    let stack_hash = crane_work_result_hash(moves, &mut stacks, n_stacks);
    assert_eq!(stack_hash, "CMZ");
    // dbg!(stacks);

    // main
    let path = Path::new("src/day05/input");
    assert!(path.exists());

    let contents = fs::read_to_string(path).unwrap().trim().to_owned();
    let (moves, n_stacks, mut stacks) = get_stacks_from_str(contents);
    //assert_eq!(stacks, ref_stacks);
    let stack_hash = crane_work_result_hash(moves, &mut stacks, n_stacks);

    println!("Stack hash: {stack_hash}");
    // regression test for refactoring
    assert_eq!(stack_hash, "QPJPLMNNR");
}

fn crane_work_result_hash(moves: Vec<Move>, stacks: &mut Vec<VecDeque<char>>, n_stacks: usize) -> String {
    // crane worker
    for move_ in moves {
        for _count in 0..move_.quantity {
            let val = stacks[move_.from_stack_idx - 1].pop_front().unwrap();
            stacks[move_.to_stack_idx - 1].push_front(val);
        }
    }
    // get hash of result
    let stack_hash: String = (0..n_stacks).into_iter().map(|i| stacks[i][0]).collect();
    stack_hash
}

fn get_example() -> String {
    let example = r#"
            [D]    
        [N] [C]    
        [Z] [M] [P]
         1   2   3 

        move 1 from 2 to 1
        move 3 from 1 to 3
        move 2 from 2 to 1
        move 1 from 1 to 2   
    "#;
    // TODO: how to simplify the following expression
    // remove first line
    let example: String = example
        .lines()
        .skip(1)
        .map(|l| l.to_owned())
        .collect::<Vec<String>>()
        .join("\n");
    example
}

fn get_stacks_from_str(example: String) -> (Vec<Move>, usize, Vec<VecDeque<char>>) {
    let parts: Vec<&str> = example.split("\n\n").collect();
    assert_eq!(parts.len(), 2);
    let (head, moves_str) = (parts[0], parts[1].trim());

    // parse body
    let re_move = regex::Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let moves: Vec<Move> = re_move
        .captures_iter(moves_str)
        .map(|cap| {
            assert_eq!(cap.len(), 4);
            Move {
                quantity: cap[1].parse().unwrap(),
                from_stack_idx: cap[2].parse().unwrap(),
                to_stack_idx: cap[3].parse().unwrap(),
            }
        })
        .collect();
    assert_eq!(
        moves.len(),
        moves_str.trim().lines().collect::<Vec<&str>>().len()
    );
    // parse header
    let header_lines: Vec<&str> = head.lines().collect();
    let n_head = header_lines.len();
    let stack_index_line = header_lines[n_head - 1];
    let stack_indices: Vec<i32> = stack_index_line
        .trim()
        .split_whitespace()
        .map(|part| part.trim().parse::<i32>().unwrap())
        .collect();
    let stack_indices_set: HashSet<i32> = stack_indices.iter().cloned().collect();
    assert_eq!(stack_indices.len(), stack_indices_set.len());
    let stack_positions: Vec<usize> = stack_indices
        .iter()
        .map(|&i| {
            let res: Vec<(usize, char)> = stack_index_line
                .chars()
                .enumerate()
                .filter(|(_num, c)| c.to_string() == i.to_string())
                .collect();
            assert_eq!(res.len(), 1);
            let num = res[0].0;
            num
        })
        .collect();
    //dbg!(&stack_positions);
    let n_stacks = stack_positions.len();
    assert_eq!(stack_indices.len(), n_stacks);
    let mut stacks: Vec<VecDeque<char>> = (0..n_stacks).map(|_i| VecDeque::new()).collect();
    // last line already consumed as stack_index_line
    //dbg!(&header_lines);
    for line_idx in 0..(n_head - 1) {
        let line = header_lines[line_idx];
        for (stack_idx, stack_pos) in stack_positions.iter().enumerate() {
            //dbg!(stack_idx, stack_pos);
            //dbg!(line);
            let chr = line.chars().nth(*stack_pos as usize).unwrap();
            if chr != ' ' {
                stacks[stack_idx].push_back(chr);
            }
        }
    }
    (moves, n_stacks, stacks)
}
