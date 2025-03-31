#[allow(dead_code)]
pub fn smaller_numbers_than_current(nums: &[usize]) -> Vec<i32> {
    nums.iter()
        .map(|&num| nums.iter().filter(|&&x| x < num).count().try_into())
        .filter_map(std::result::Result::ok)
        .collect()
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let res = smaller_numbers_than_current(&[8, 1, 2, 2, 3]);
        assert_eq!(res, vec![4, 0, 1, 1, 3]);
        let res = smaller_numbers_than_current(&[6, 5, 4, 8]);
        assert_eq!(res, vec![2, 1, 0, 3]);
        let res = smaller_numbers_than_current(&[7, 7, 7, 7]);
        assert_eq!(res, vec![0, 0, 0, 0]);
    }
}
