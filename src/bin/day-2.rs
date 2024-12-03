use aoc_2024::checks::{changes_direction, too_different};
use itertools::Itertools;
fn main() {
    let contents = include_str!("../../inputs/day-2/reports.txt");
    let safe_reports: usize = contents
        .lines()
        .map(|line| {
            line
                .split_whitespace()
                .filter_map(|s| s.parse::<i32>().ok())
        })
        .filter(|line| {
            let changes_direction: bool = line
                .clone()
                .tuple_windows::<(i32, i32, i32)>()
                .any(|window| changes_direction(&window));
            let too_different: bool = line
                .clone()
                .tuple_windows::<(i32, i32)>()
                .any(|window| too_different(&window));

            let is_safe = !(changes_direction || too_different);
            is_safe
        })
        .count();
    println!("Safe reports raw: {}", safe_reports);
    println!("Safe reports with Problem Dampener: {}", safe_reports);
}
