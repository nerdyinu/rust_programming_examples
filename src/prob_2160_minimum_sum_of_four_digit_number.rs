#[allow(dead_code)]
pub fn minimum_sum(num: i32) -> i32 {
    let mut digits: Vec<i32> = num
        .to_string()
        .chars()
        .filter_map(|c| c.to_digit(10).and_then(|digit| i32::try_from(digit).ok()))
        .collect();

    digits.sort_unstable();

    let new1 = digits[0] * 10 + digits[2];
    let new2 = digits[1] * 10 + digits[3];

    new1 + new2
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let res = minimum_sum(1234);
        assert_eq!(res, 37);
    }
}
