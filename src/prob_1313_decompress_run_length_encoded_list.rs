#![allow(dead_code)]
pub fn decompress_rl_elist(nums: &[usize]) -> Result<Vec<usize>, &'static str> {
    nums.chunks(2)
        .try_fold(Vec::new(), |mut acc, chunk| match chunk {
            [freq, val] => {
                acc.extend(std::iter::repeat(*val).take(*freq));
                Ok(acc)
            }
            _ => Err("Invalid input"),
        })
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let nums = vec![1, 2, 3, 4];
        let res = decompress_rl_elist(&nums);
        assert_eq!(res, Ok(vec![2, 4, 4, 4]));
        let nums = vec![1, 1, 2, 2];
        let res = decompress_rl_elist(&nums);
        assert_eq!(res, Ok(vec![1, 2, 2]));
    }
}
