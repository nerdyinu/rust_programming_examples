use std::collections::HashMap;

#[allow(dead_code)]
pub fn num_identical_pairs(nums: &[i32]) -> usize {
    let mut count = 0;
    let mut pairs = HashMap::new();
    for &num in nums {
        pairs
            .entry(num)
            .and_modify(|v| {
                count += *v;
                *v += 1;
            })
            .or_insert(1);
    }
    count
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
