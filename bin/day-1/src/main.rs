use std::fs;
use std::iter::zip;
use itertools::Itertools;

fn main() {
    let contents = fs::read_to_string("list.txt").expect("Failed to read file");
    let (mut left, mut right): (Vec<i32>, Vec<i32>) = contents
        .lines()
        .flat_map(|line| line.split("\t"))
        .filter_map(|s| s.parse::<i32>().ok())
        .tuples()
        .unzip();
    left.sort();
    right.sort();

    let result: u32 = zip(left, right)
        .map(|(a, b)| a.abs_diff(b))
        .sum();


    
    
    println!("{}", result);
}