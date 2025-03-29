pub fn shuffle(nums: &[i32], n: usize) -> Result<Vec<i32>, &'static str> {
    if nums.is_empty() {
        return Err("Input array is empty");
    }

    if nums.len() > 500 {
        return Err("Input array size exceeds 500");
    }

    if nums.len() != 2 * n {
        return Err("Input array size is not equal to 2n");
    }
    let (first, second) = nums.split_at(n);
    Ok(first
        .iter()
        .zip(second.iter())
        .flat_map(|(&x, &y)| vec![x, y])
        .collect())
}
mod tests {
    use super::*;

    #[test]
    fn test_valid_input() {
        let res = shuffle(&[2, 5, 1, 3, 4, 7], 3).unwrap();
        assert_eq!(res, vec![2, 3, 5, 4, 1, 7]);
    }

    #[test]
    fn test_empty_array() {
        let result = shuffle(&[], 0);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Input array is empty");
    }

    #[test]
    fn test_array_too_large() {
        // Create an array with 501 elements
        let large_array = vec![0; 501];
        let result = shuffle(&large_array, 250);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Input array size exceeds 500");
    }

    #[test]
    fn test_array_size_not_2n() {
        // Array with 5 elements cannot be evenly split
        let result = shuffle(&[1, 2, 3, 4, 5], 2);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Input array size is not equal to 2n");
    }
}
