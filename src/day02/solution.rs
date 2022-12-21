use std::collections::HashMap;
use std::fs;
use std::hash::Hash;
use std::path::Path;

// total score = "share score" + "outcome score"

// C Y
// C X
// A Z
// B X
// B Z

// TODO: why copy
#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissor,
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
enum Outcome {
    Win,
    Draw,
    Loose,
}

// you are player1 getting shape1
fn game(shape1: Shape, shape0: Shape) -> i32 {
    // shape score: 1 for Rock, 2 for Paper, and 3 for Scissors
    let shape_to_score: HashMap<Shape, i32> =
        HashMap::from([(Shape::Rock, 1), (Shape::Paper, 2), (Shape::Scissor, 3)]);
    // outcome score: 0 if you lost, 3 if the round was a draw, and 6 if you won
    // TODO: use enum for score
    match (shape1, shape0) {
        (Shape::Rock, Shape::Rock)
        | (Shape::Paper, Shape::Paper)
        | (Shape::Scissor, Shape::Scissor) => shape_to_score[&shape1] + 3,
        (Shape::Rock, Shape::Paper) => shape_to_score[&shape1] + 0,
        (Shape::Paper, Shape::Rock) => shape_to_score[&shape1] + 6, // win
        (Shape::Rock, Shape::Scissor) => shape_to_score[&shape1] + 6, // win
        (Shape::Scissor, Shape::Rock) => shape_to_score[&shape1] + 0,
        (Shape::Paper, Shape::Scissor) => shape_to_score[&shape1] + 0,
        (Shape::Scissor, Shape::Paper) => shape_to_score[&shape1] + 6, // win
    }
}

fn move_(outcome: Outcome, shape0: Shape) -> Shape {
    match (outcome, shape0) {
        (Outcome::Draw, _) => shape0,
        (Outcome::Win, Shape::Rock) => Shape::Paper,
        (Outcome::Loose, Shape::Rock) => Shape::Scissor,
        (Outcome::Win, Shape::Paper) => Shape::Scissor,
        (Outcome::Loose, Shape::Paper) => Shape::Rock,
        (Outcome::Win, Shape::Scissor) => Shape::Rock,
        (Outcome::Loose, Shape::Scissor) => Shape::Paper,
    }
}

pub fn main() {
    // TODO: howto create constant global hashmaps or something similar
    // A for Rock, B for Paper, and C for Scissors
    let col0_to_shape: HashMap<&str, Shape> = HashMap::from([
        ("A", Shape::Rock),
        ("B", Shape::Paper),
        ("C", Shape::Scissor),
    ]);
    // X for Rock, Y for Paper, and Z for Scissors
    let col1_to_shape: HashMap<&str, Shape> = HashMap::from([
        ("X", Shape::Rock),
        ("Y", Shape::Paper),
        ("Z", Shape::Scissor),
    ]);

    assert_eq!(game(col1_to_shape["Y"], col0_to_shape["A"]), 8);
    assert_eq!(game(col1_to_shape["X"], col0_to_shape["B"]), 1);
    assert_eq!(game(col1_to_shape["Z"], col0_to_shape["C"]), 6);

    let path = Path::new("src/day02/input");
    assert!(path.exists());

    let contents = fs::read_to_string(path).unwrap();

    let mut total_score = 0;
    for line in contents.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        assert_eq!(parts.len(), 2);
        let (col0, col1) = (parts[0], parts[1]);
        let score = game(col1_to_shape[col1], col0_to_shape[col0]);
        total_score += score;
    }

    println!("Total Score: {total_score}");
    // regression test for refactoring
    assert_eq!(total_score, 13446);

    // part two
    let col1_to_outcome: HashMap<&str, Outcome> = HashMap::from([
        ("X", Outcome::Loose),
        ("Y", Outcome::Draw),
        ("Z", Outcome::Win),
    ]);

    let shape0 = col0_to_shape["A"];
    assert_eq!(game(move_(col1_to_outcome["Y"], shape0), shape0), 4);
    let shape0 = col0_to_shape["B"];
    assert_eq!(game(move_(col1_to_outcome["X"], shape0), shape0), 1);
    let shape0 = col0_to_shape["C"];
    assert_eq!(game(move_(col1_to_outcome["Z"], shape0), shape0), 7);

    let mut total_score = 0;
    for line in contents.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        assert_eq!(parts.len(), 2);
        let (col0, col1) = (parts[0], parts[1]);
        let shape0 = col0_to_shape[col0];
        let shape1 = move_(col1_to_outcome[col1], shape0);
        let score = game(shape1, shape0);
        total_score += score;
    }

    println!("New Strategy Score: {total_score}");
    // regression test for refactoring
    assert_eq!(total_score, 13509);
}
