#![feature(iter_array_chunks)]

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day09;
mod day12;
mod day13;
mod day21;

use crate::day01::main as day01_main;
use crate::day02::main as day02_main;
use crate::day03::main as day03_main;
use crate::day04::main as day04_main;
use crate::day05::main as day05_main;
use crate::day06::main as day06_main;
use crate::day07::main as day07_main;
use crate::day09::main as day09_main;
use crate::day12::main as day12_main;
use crate::day13::main as day13_main;
use crate::day21::main as day21_main;

fn main () {
    day01_main();
    day02_main();
    day03_main();
    day04_main();
    day05_main();
    day06_main();
    day07_main();
    day09_main();
    day12_main();
    day13_main();
    day21_main();
}