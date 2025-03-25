#[allow(dead_code)]
pub fn build_array(nums: &[i32]) -> Vec<i32> {
    nums.iter()
        .filter_map(|&num| {
            usize::try_from(num)
                .ok()
                .and_then(|num| nums.get(num).copied())
        })
        .collect::<Vec<i32>>()
}
#[cfg(test)]
mod tests {
    use super::build_array;
    #[test]
    fn test() {
        let res = build_array(&[0, 2, 1, 5, 3, 4]);
        assert_eq!(res, vec![0, 1, 2, 4, 5, 3]);
    }
}
