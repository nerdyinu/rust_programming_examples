#![allow(dead_code)]
pub fn num_jewels_in_stones(jewels: &str, stones: &str) -> Result<usize, &'static str> {
    if jewels.is_empty() || stones.is_empty() {
        return Err("Input strings cannot be empty");
    }
    Ok(stones.chars().filter(|&c| jewels.contains(c)).count())
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_num_jewels_in_stones() {
        let jewels = "aA";
        let stones = "aAAbbbb";
        assert_eq!(num_jewels_in_stones(jewels, stones), Ok(3));
    }

    #[test]
    fn test_empty_jewels() {
        let jewels = "";
        let stones = "aAAbbbb";
        assert_eq!(
            num_jewels_in_stones(jewels, stones),
            Err("Input strings cannot be empty")
        );
    }

    #[test]
    fn test_empty_stones() {
        let jewels = "aA";
        let stones = "";
        assert_eq!(
            num_jewels_in_stones(jewels, stones),
            Err("Input strings cannot be empty")
        );
    }
}
