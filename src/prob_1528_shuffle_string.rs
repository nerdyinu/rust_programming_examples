#![allow(dead_code)]
pub fn restore_string(s: &str, indices: &[usize]) -> String {
    let mut result = vec![' '; s.len()];
    for (i, char) in indices.iter().zip(s.chars()) {
        result[*i] = char;
    }
    result.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_restore_string() {
        assert_eq!(
            restore_string("codeleet", &[4, 5, 6, 7, 0, 2, 1, 3]),
            "leetcode".to_string()
        );
        assert_eq!(restore_string("abc", &[0, 1, 2]), "abc".to_string());
        assert_eq!(
            restore_string("aiohn", &[3, 1, 4, 2, 0]),
            "nihao".to_string()
        );
        assert_eq!(restore_string("a", &[0]), "a".to_string());
        assert_eq!(restore_string("art", &[1, 0, 2]), "rat".to_string());
    }
}
