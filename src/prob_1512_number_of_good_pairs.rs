#[allow(dead_code)]
pub fn num_identical_pairs(nums: &[i32]) -> usize {
    nums.iter()
        .enumerate()
        .flat_map(|(i, &x)| nums.iter().skip(i + 1).filter(move |&&y| x == y))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let nums = &[1, 2, 3, 1, 1, 3];
        assert_eq!(num_identical_pairs(nums), 4);
    }
}
