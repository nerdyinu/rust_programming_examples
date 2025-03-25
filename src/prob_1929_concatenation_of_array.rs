#[allow(dead_code)]
pub fn get_concatenation(nums: &[i32]) -> Vec<i32> {
    let mut ans = Vec::with_capacity(nums.len() * 2);
    ans.extend_from_slice(nums);
    ans.extend_from_slice(nums);
    ans
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let res = get_concatenation(&[1, 2, 1]);
        assert_eq!(res, vec![1, 2, 1, 1, 2, 1]);
    }
}
