#[allow(dead_code)]
pub fn subtract_product_and_sum(n: i32) -> i32 {
    let mut num = n;
    let mut sum = 0;
    let mut prod = 1;
    while num > 0 {
        let digit = num % 10;
        sum += digit;
        prod *= digit;
        num /= 10;
    }
    prod - sum
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let res = subtract_product_and_sum(4421);
        assert_eq!(res, 21);
    }
}
