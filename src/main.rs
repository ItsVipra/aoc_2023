use chrono::{Datelike, Utc};
use clap::Parser;
use aoc_2023::*;

mod days;

fn main() {
    let args = Args::parse();
    // println!("{:?}", args);

    let day = args.day.unwrap_or(Utc::now().day());
    let input_path = args.input.unwrap_or(format!("./input/{}.txt", day));

    let input_data = aoc_2023::load_input(&input_path);

    match day {
        1 => {
            crate::days::day_1::exec(input_data);
        }
        2 => {
            days::day_2::exec(input_data);
        }
        3 => {
            days::day_3::exec(input_data);
        }
        4..=25 => todo!(),
        _ => {panic!("i dunno this date: {}", day)} }
}