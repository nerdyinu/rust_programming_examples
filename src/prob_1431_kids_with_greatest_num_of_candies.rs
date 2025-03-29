#[allow(dead_code)]
pub fn kids_with_candies(candies: &[i32], extra_candies: i32) -> Vec<bool> {
    let max_candies = candies.iter().max().unwrap();
    candies
        .iter()
        .map(|&c| c + extra_candies >= *max_candies)
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let res = kids_with_candies(&[2, 3, 5, 1, 3], 3);
        assert_eq!(res, vec![true, true, true, false, true]);
    }
}
