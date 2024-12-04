use regex::Regex;
fn main() {
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    let haystack = include_str!("../../inputs/day-3/memory.txt");
    let res: i32 = haystack
        .lines()
        .fold(0, |acc, line| {
            acc + re
                .captures_iter(line)
                .map(|caps| caps.extract())
                .fold(0, |line_acc, (_, [left, right])| {
                    line_acc + left.parse::<i32>().unwrap() * right.parse::<i32>().unwrap()
                })
        });
    
    println!("{}", res);
}