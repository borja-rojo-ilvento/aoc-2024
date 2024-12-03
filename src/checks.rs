pub fn too_different(left: i32, right: i32) -> bool {
    let diff: u32 = left.abs_diff(right);
    diff >= 1 && diff <= 3
}

pub fn changes_direction(left: i32, middle: i32, right: i32) -> bool {
    (left < middle && middle > right) || (left > middle && middle < right)
}
