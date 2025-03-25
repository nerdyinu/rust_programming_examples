pub fn running_sum(nums: &[i32]) -> Vec<i32> {
    nums.iter()
        .scan(0, |acc, &x| {
            *acc += x;
            Some(*acc)
        })
        .collect()
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let res = running_sum(&[1, 2, 3, 4]);
        assert_eq!(res, vec![1, 3, 6, 10]);
    }
}
