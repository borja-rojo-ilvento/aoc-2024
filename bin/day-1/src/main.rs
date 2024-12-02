use std::iter::zip;
use itertools::Itertools;

fn main() {
    let contents = include_str!("list.txt");
    // println!("{:?}", contents);
    // let lines = contents.lines();
    // println!("{:?}", lines);    
    let (mut left, mut right): (Vec<i32>, Vec<i32>) = contents
        .split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .tuples()
        .unzip();
    left.sort();
    right.sort();

    let result: u32 = zip(left, right)
        .map(|(left, right)| left.abs_diff(right))
        .sum();
    println!("{}", result);
}