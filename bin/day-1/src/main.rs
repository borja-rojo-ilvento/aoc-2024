use std::fs;
use std::iter::zip;
use itertools::Itertools;

fn main() {
    let contents = include_str!("list.txt");
    let (mut left, mut right): (Vec<i32>, Vec<i32>) = contents
        .lines()
        .flat_map(|line| line.split("\t"))
        .filter_map(|s| s.parse::<i32>().ok())
        .tuples()
        .unzip();
    left = left.sort();
    right = right.sort();

    let result: u32 = zip(left.clone(), right.clone())
        .map(|(a, b)| a.abs_diff(b))
        .sum();


    println!("{:?}", left);
    println!("{:?}", right);
    println!("{}", result);
}