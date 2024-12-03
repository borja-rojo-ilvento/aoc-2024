pub fn too_different(&(left, right): &(i32, i32)) -> bool {
    let diff: u32 = left.abs_diff(right);
    diff < 1 || diff > 3
}

pub fn changes_direction(&(left, middle, right): &(i32, i32, i32)) -> bool {
    (left < middle && middle > right) || (left > middle && middle < right)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_too_different() {
        // Too far
        assert!(too_different(&(5, 12)), "Should be too different: ascending 5->12"); 
        assert!(too_different(&(11, 7)), "Should be too different: descending 11->7");

        assert!(too_different(&(10, 10)), "Should be too different: same value 10"); 
        assert!(too_different(&(25, 25)), "Should be too different: same value 25");

        // Within bounds
        assert!(!too_different(&(7, 8)), "Should be within bounds: ascending by 1"); 
        assert!(!too_different(&(9, 11)), "Should be within bounds: ascending by 2");
        assert!(!too_different(&(12, 15)), "Should be within bounds: ascending by 3");

        assert!(!too_different(&(8, 7)), "Should be within bounds: descending by 1");
        assert!(!too_different(&(11, 9)), "Should be within bounds: descending by 2");
        assert!(!too_different(&(15, 12)), "Should be within bounds: descending by 3");
    }

    #[test]
    fn test_changes_direction() {
        // Down-up-down
        assert!(changes_direction(&(2, 1, 2)), "Should change direction: valley pattern 2->1->2");
        assert!(changes_direction(&(200, 100, 200)), "Should change direction: valley pattern 200->100->200");
        // Up-down-up
        assert!(changes_direction(&(1, 2, 1)), "Should change direction: peak pattern 1->2->1");
        assert!(changes_direction(&(100, 200, 100)), "Should change direction: peak pattern 100->200->100");
        // Ascending
        assert!(!changes_direction(&(1, 2, 3)), "Should not change direction: ascending similar 1->2->3"); 
        assert!(!changes_direction(&(100, 200, 300)), "Should not change direction: ascending dissimilar 100->200->300");
        // Descending
        assert!(!changes_direction(&(3, 2, 1)), "Should not change direction: descending similar 3->2->1");
        assert!(!changes_direction(&(300, 200, 100)), "Should not change direction: descending dissimilar 300->200->100")
    }
}
