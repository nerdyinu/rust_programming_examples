#[allow(dead_code)]
pub fn number_of_steps(num: i32) -> i32 {
    let mut curr = num;
    let mut count = 0;
    while curr != 0 {
        if curr % 2 == 0 {
            curr >>= 1;
        } else {
            curr -= 1;
        }
        count += 1;
    }
    count
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(number_of_steps(14), 6);
        assert_eq!(number_of_steps(8), 4);
        assert_eq!(number_of_steps(123), 12);
    }
}
