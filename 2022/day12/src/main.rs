use std::cmp::{min, max};
use std::collections::HashSet;
use std::{fs, path::Path};

#[allow(dead_code)]
fn chr(i: u32) -> char {
    i.try_into().unwrap()
}

// TODO: optimize through dictionary
fn ord(c: char) -> i32 {
    let num: u32 = match c {
        'S' => 'a'.into(),
        'E' => 'z'.into(),
        _ => c.into(),
    };
    i32::try_from(num).unwrap()
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Pos {
    x: usize,
    y: usize,
}

fn main() {
    let example = r#"
        Sabqponm
        abcryxxl
        accszExk
        acctuvwj
        abdefghi   
    "#
    .trim();

    // test
    let (initial_journey_offset, finished_journeys) = get_journies(example);
    let shortest_journey_len = get_shortes_journey_len(finished_journeys, initial_journey_offset);
    assert_eq!(shortest_journey_len, 31);

    // main
    let path = Path::new("input");
    assert!(path.exists());
    let contents = fs::read_to_string(path).unwrap();
    let (initial_journey_offset, finished_journeys) = get_journies(contents.trim());
    let shortest_journey_len = get_shortes_journey_len(finished_journeys, initial_journey_offset);

    println!("Shortest journey lenght: {}", shortest_journey_len);
}

fn get_shortes_journey_len(
    finished_journeys: Vec<Vec<Pos>>,
    initial_journey_offset: usize,
) -> usize {
    let shortest_journey_len = finished_journeys
        .iter()
        .map(|trip| trip.len())
        .reduce(min)
        .unwrap()
        - initial_journey_offset;
    shortest_journey_len
}

fn get_journies(example: &str) -> (usize, Vec<Vec<Pos>>) {
    let lines: Vec<Vec<char>> = example
        .lines()
        .map(|l| l.trim().chars().collect())
        .collect();
    let y_len = lines.len();
    let x_len = lines[0].len();
    dbg!(y_len);
    dbg!(x_len);
    dbg!(x_len * y_len);
    let y_len_i32 = i32::try_from(y_len).unwrap();
    let x_len_i32 = i32::try_from(x_len).unwrap();
    let start = get_pos_of_char(&lines, 'S');
    let target = get_pos_of_char(&lines, 'E');
    dbg!(start);
    dbg!(target);
    let mut journey: Vec<Vec<Pos>> = vec![vec![start]];
    let initial_journey_offset = journey.len();
    let mut loop_counter = 0;
    let mut prune_idxs: Vec<usize> = Vec::new();
    loop {
        dbg!(journey.len());
        if !prune_idxs.is_empty() {
            for prune_idx in prune_idxs.into_iter().rev() {
                let _ = &journey.remove(prune_idx);
            }
            prune_idxs = Vec::new();
        }
        let journey_len = journey.len();
        dbg!(journey_len);
        dbg!(loop_counter);
        let visited: HashSet<Pos> = journey.iter().flatten().cloned().collect();
        let visited_minus_one: HashSet<Pos> = journey
            .iter()
            .flat_map(|trip| {
                let len_minus_one = max(i32::try_from(trip.len()).unwrap() - 2, 0i32);
                trip.iter().take(usize::try_from(len_minus_one).unwrap()).cloned().collect::<Vec<Pos>>()
            })
            .collect();
        dbg!(visited.len());
        let mut signalize_break: bool = false;
        for trip_idx in 0..journey_len {
            let trip_len = journey[trip_idx].len();
            if trip_len >= x_len * y_len {
                continue;
            }
            let current = journey[trip_idx][trip_len - 1];
            if current == target {
                signalize_break = true;
                break;
            }
            if visited_minus_one.contains(&current) {
                prune_idxs.push(trip_idx);
                continue;
            }
            // let current_char = lines[current.y][current.x];
            let new_positions = try_to_step(&lines, current, x_len_i32, y_len_i32, &visited);
            if new_positions.is_empty() {
                // dead end
                prune_idxs.push(trip_idx);
                continue;
            }
            let trip_clone = journey[trip_idx].clone();
            for (num, pos) in new_positions.iter().enumerate() {
                // TODO: optimize away this if
                let trip_ref = if num == 0 {
                    &mut journey[trip_idx]
                } else {
                    journey.push(trip_clone.clone());
                    let journey_len = journey.len();
                    &mut journey[journey_len - 1]
                };
                trip_ref.push(*pos);
            }
        }
        if signalize_break {
            println!("Break due to signalize_break");
            break;
        }
        loop_counter += 1;
        if loop_counter > x_len * y_len {
            println!("Probably endless loop, loopCounter = {loop_counter}");
        }
    }
    let finished_journeys: Vec<Vec<Pos>> = journey
        .iter()
        .filter(|trip| trip[trip.len() - 1] == target)
        .cloned()
        .collect();
    (initial_journey_offset, finished_journeys)
}

fn try_to_step(
    lines: &[Vec<char>],
    current: Pos,
    x_len_i32: i32,
    y_len_i32: i32,
    visited: &HashSet<Pos>,
) -> Vec<Pos> {
    let mut steps: Vec<Pos> = Vec::new();
    let current_elevation = ord(lines[current.y][current.x]);
    for step in [1i32, -1i32] {
        let x_new = i32::try_from(current.x).unwrap() + step;
        if x_new < x_len_i32 && x_new >= 0 {
            let new_pos = Pos {
                x: usize::try_from(x_new).unwrap(),
                y: current.y,
            };
            let new_elevation = ord(lines[new_pos.y][new_pos.x]);
            if new_elevation - current_elevation <= 1 && !visited.contains(&new_pos) {
                steps.push(new_pos)
            }
        }
        let y_new_i32 = i32::try_from(current.y).unwrap() + step;
        if y_new_i32 < y_len_i32 && y_new_i32 >= 0 {
            let new_pos = Pos {
                x: current.x,
                y: usize::try_from(y_new_i32).unwrap(),
            };
            let new_elevation = ord(lines[new_pos.y][new_pos.x]);
            if new_elevation - current_elevation <= 1 && !visited.contains(&new_pos) {
                steps.push(new_pos)
            }
        }
    }
    steps
}

fn get_pos_of_char(lines: &[Vec<char>], character: char) -> Pos {
    for (y, _) in lines.iter().enumerate() {
        for (x, _) in lines[y].iter().enumerate() {
            if lines[y][x] == character {
                return Pos { x, y };
            }
        }
    }
    panic!("Could not get position of character={character}");
}
