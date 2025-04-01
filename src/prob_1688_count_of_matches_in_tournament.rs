pub fn number_of_matches(n: i32) -> i32 {
    let mut curr = n;
    let mut count = 0;
    while curr != 1 {
        if curr % 2 == 0 {
            curr /= 2;
            count += curr;
        } else {
            curr = (curr + 1) / 2;
            count += curr - 1;
        }
    }
    count
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_of_matches() {
        assert_eq!(number_of_matches(7), 6);
        assert_eq!(number_of_matches(14), 13);
    }
}
