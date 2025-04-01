#[allow(dead_code)]
pub fn final_value_after_operations(operations: &[String]) -> Result<i32, &'static str> {
    if operations.is_empty() {
        return Err("Input array cannot be empty");
    }
    Ok(operations.iter().fold(0, |acc, curr| {
        let trimmed = curr.trim_matches('X');
        let num = match trimmed.chars().next() {
            Some('+') => 1,
            Some('-') => -1,
            _ => 0,
        };
        acc + num
    }))
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_operations() {
        let operations: Vec<String> = vec![];
        let result = final_value_after_operations(&operations);
        assert_eq!(result, Err("Input array cannot be empty"));
    }

    #[test]
    fn test_basic_operations() {
        let operations: Vec<String> = vec!["X++".to_string(), "++X".to_string(), "X--".to_string()];
        assert_eq!(final_value_after_operations(&operations), Ok(1));
    }
}
