use aoc_2024::word_search::{Grid, Direction};

fn main() {
    println!("Advent of Code 2023 - Day 4");
    
    let input = include_str!("../../inputs/day-4/word-search.txt");
    
    // Part 1
    let part1_result = solve_part1(input);
    println!("Part 1: {}", part1_result);
    
    // Part 2
    let part2_result = solve_part2(input);
    println!("Part 2: {}", part2_result);
}

fn solve_part1(input: &str) -> u32 {
    let grid = Grid::new(input);
    let results = grid.find_all_words("XMAS");
    
    println!("Found {} occurrences of XMAS:", results.len());
    for (row, col, direction) in &results {
        println!("  - At row {}, col {}, direction {:?}", row, col, direction);
    }
    
    results.len() as u32
}

fn solve_part2(input: &str) -> u32 {
    let grid = Grid::new(input);
    // TODO: Implement part 2
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
XMAS
ABCD
EFGH";

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(TEST_INPUT), 1);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(TEST_INPUT), 0);
    }
} 