#[allow(dead_code)]
pub fn kids_with_candies(candies: &[i32], extra_candies: i32) -> Result<Vec<bool>, &str> {
    if candies.is_empty() {
        return Err("Input array cannot be empty");
    }

    let max_candies = candies
        .iter()
        .max()
        .expect("should not be None since array is guaranteed to be non-empty");

    Ok(candies
        .iter()
        .map(|&c| c + extra_candies >= *max_candies)
        .collect())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let res = kids_with_candies(&[2, 3, 5, 1, 3], 3);
        assert_eq!(res, Ok(vec![true, true, true, false, true]));
    }
    #[test]
    fn test_empty_array() {
        let res = kids_with_candies(&[], 3);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "Input array cannot be empty");
    }
}
