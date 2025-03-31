#![allow(dead_code)]
pub fn decode(encoded: &[i32], first: i32) -> Vec<i32> {
    let mut result = Vec::with_capacity(encoded.len() + 1);
    result.push(first);

    encoded.iter().fold(first, |prev, &encoded_val| {
        let next = prev ^ encoded_val;
        result.push(next);
        next
    });

    result
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let encoded = vec![1, 2, 3];
        let first = 1;
        let res = decode(&encoded, first);
        assert_eq!(res, vec![1, 0, 2, 1]);
    }
}
