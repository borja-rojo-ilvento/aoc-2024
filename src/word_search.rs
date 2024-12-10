use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Direction {
    fn to_delta(self) -> (i32, i32) {
        match self {
            Direction::North => (-1, 0),
            Direction::NorthEast => (-1, 1),
            Direction::East => (0, 1),
            Direction::SouthEast => (1, 1),
            Direction::South => (1, 0),
            Direction::SouthWest => (1, -1),
            Direction::West => (0, -1),
            Direction::NorthWest => (-1, -1),
        }
    }
}

pub struct Grid {
    data: Vec<char>,
    rows: usize,
    cols: usize,
}

impl Grid {
    pub fn new(input: &str) -> Self {
        let data: Vec<char> = input.lines()
            .flat_map(|line| line.chars())
            .collect();
        
        let rows = input.lines().count();
        let cols = if rows > 0 {
            input.lines().next().unwrap().len()
        } else {
            0
        };

        Self { data, rows, cols }
    }

    pub fn get(&self, row: usize, col: usize) -> Option<char> {
        if row < self.rows && col < self.cols {
            Some(self.data[row * self.cols + col])
        } else {
            None
        }
    }

    pub fn iter_from(&self, start_row: i32, start_col: i32, direction: Direction) -> DirectionalIterator {
        let (dx, dy) = direction.to_delta();
        DirectionalIterator {
            grid: self,
            current_row: start_row,
            current_col: start_col,
            dx,
            dy,
        }
    }

    pub fn find_all_words(&self, word: &str) -> Vec<(usize, usize, Direction)> {
        let word_chars: Vec<char> = word.chars().collect();
        let mut results = Vec::new();
        
        for row in 0..self.rows {
            for col in 0..self.cols {
                for direction in [
                    Direction::North,
                    Direction::NorthEast,
                    Direction::East,
                    Direction::SouthEast,
                    Direction::South,
                    Direction::SouthWest,
                    Direction::West,
                    Direction::NorthWest,
                ] {
                    let chars: Vec<char> = self.iter_from(row as i32, col as i32, direction)
                        .take(word.len())
                        .filter_map(|x| Some(x))
                        .collect();
                    if chars.len() == word.len() && chars.iter().zip(word_chars.iter()).all(|(a, b)| a == b) {
                        results.push((row, col, direction));
                    }
                }
            }
        }
        results
    }
}

pub struct DirectionalIterator<'a> {
    grid: &'a Grid,
    current_row: i32,
    current_col: i32,
    dx: i32,
    dy: i32,
}

impl<'a> Iterator for DirectionalIterator<'a> {
    type Item = char;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_row < 0 || 
           self.current_row >= self.grid.rows as i32 || 
           self.current_col < 0 || 
           self.current_col >= self.grid.cols as i32 {
            return None;
        }
        
        let c = self.grid.get(self.current_row as usize, self.current_col as usize);
        self.current_row += self.dx;
        self.current_col += self.dy;
        c
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_creation() {
        let input = "ABC\nDEF\nGHI";
        let grid = Grid::new(input);
        assert_eq!(grid.rows, 3);
        assert_eq!(grid.cols, 3);
        assert_eq!(grid.get(0, 0), Some('A'));
        assert_eq!(grid.get(1, 1), Some('E'));
        assert_eq!(grid.get(2, 2), Some('I'));
    }

    #[test]
    fn test_directional_iterator() {
        let input = "ABC\nDEF\nGHI";
        let grid = Grid::new(input);
        
        // Test horizontal iteration (East)
        let chars: Vec<char> = grid.iter_from(0, 0, Direction::East).collect();
        assert_eq!(chars, vec!['A', 'B', 'C']);
        
        // Test vertical iteration (South)
        let chars: Vec<char> = grid.iter_from(0, 0, Direction::South).collect();
        assert_eq!(chars, vec!['A', 'D', 'G']);
        
        // Test diagonal iteration (SouthEast)
        let chars: Vec<char> = grid.iter_from(0, 0, Direction::SouthEast).collect();
        assert_eq!(chars, vec!['A', 'E', 'I']);
    }

    #[test]
    fn test_boundary_iteration() {
        let input = "ABC\nDEF\nGHI";
        let grid = Grid::new(input);
        
        // Test that North direction from row 0 stops after one char
        let chars: Vec<char> = grid.iter_from(0, 0, Direction::North).collect();
        assert_eq!(chars, vec!['A']);
        
        // Test that West direction from col 0 stops after one char
        let chars: Vec<char> = grid.iter_from(0, 0, Direction::West).collect();
        assert_eq!(chars, vec!['A']);
        
        // Test that East direction from last column stops after one char
        let chars: Vec<char> = grid.iter_from(0, 2, Direction::East).collect();
        assert_eq!(chars, vec!['C']);
        
        // Test that South direction from last row stops after one char
        let chars: Vec<char> = grid.iter_from(2, 0, Direction::South).collect();
        assert_eq!(chars, vec!['G']);
    }

    #[test]
    fn test_find_all_words() {
        let input = "ABCABC\nDEFDEF\nGHIGHI";
        let grid = Grid::new(input);
        
        // Test finding multiple occurrences
        let results = grid.find_all_words("ABC");
        assert_eq!(results.len(), 2);
        assert!(results.contains(&(0, 0, Direction::East)));
        assert!(results.contains(&(0, 3, Direction::East)));
    }
} 