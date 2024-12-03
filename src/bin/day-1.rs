use std::iter::zip;
use itertools::Itertools;
use std::collections::HashMap;
fn main() {
    let contents = include_str!("../../inputs/day-1/list.txt");
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

    let result: u32 = zip(left.clone(), right.clone())
        .map(|(left, right)| left.abs_diff(right))
        .sum();
    println!("List difference: {}", result);

    let sim_map: HashMap<i32, usize> = right
        .iter()
        .fold(HashMap::new(), |mut map, num| {
            *map.entry(*num).or_insert(0) += 1;
            map
        });

    let result: i32 = left
        .iter()
        .map(|num| num * *sim_map.get(num).unwrap_or(&0) as i32)
        .sum();
    println!("Similarity: {}", result);
}
