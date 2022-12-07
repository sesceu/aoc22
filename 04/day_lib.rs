use std::cmp;

pub fn contains(start1: u32, end1: u32, start2: u32, end2: u32) -> bool {
    let min = cmp::min(start1, start2);
    let max = cmp::max(end1, end2);

    let is_2_contained_in_1 = min == start1 && max == end1;
    let is_1_contained_in_2 = min == start2 && max == end2;

    return is_2_contained_in_1 || is_1_contained_in_2;
}

pub fn overlaps(start1: u32, end1: u32, start2: u32, end2: u32) -> bool {
    let min = cmp::min(start1, start2);
    let max = cmp::max(end1, end2);

    let len1 = end1 - start1;
    let len2 = end2 - start2;
    return (max - min) <= (len1 + len2);
}
