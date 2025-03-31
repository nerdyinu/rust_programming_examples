#![allow(dead_code)]
pub fn count_matches(
    items: &[Vec<String>],
    rule_key: &str,
    rule_value: &str,
) -> Result<i32, &'static str> {
    let rule_idx = match rule_key {
        "type" => 0,
        "color" => 1,
        "name" => 2,
        _ => return Err("Invalid rule key"),
    };
    Ok(items.iter().fold(0i32, |acc, item| {
        if item[rule_idx] == rule_value {
            acc + 1
        } else {
            acc
        }
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_count_matches() {
        let items = vec![
            vec!["phone".to_string(), "blue".to_string(), "pixel".to_string()],
            vec![
                "computer".to_string(),
                "silver".to_string(),
                "lenovo".to_string(),
            ],
            vec![
                "phone".to_string(),
                "gold".to_string(),
                "iphone".to_string(),
            ],
        ];

        assert_eq!(count_matches(&items, "type", "phone"), Ok(2));
        assert_eq!(count_matches(&items, "color", "silver"), Ok(1));
        assert_eq!(count_matches(&items, "name", "pixel"), Ok(1));
        assert_eq!(count_matches(&items, "type", "tablet"), Ok(0));
        assert_eq!(count_matches(&items, "color", "blue"), Ok(1));
        assert_eq!(count_matches(&items, "name", "lenovo"), Ok(1));
        assert_eq!(count_matches(&items, "color", "gold"), Ok(1));
        assert_eq!(count_matches(&items, "type", "computer"), Ok(1));
        assert_eq!(count_matches(&items, "name", "iphone"), Ok(1));
        assert_eq!(count_matches(&items, "color", "red"), Ok(0));
        assert_eq!(
            count_matches(&items, "invalid", "value"),
            Err("Invalid rule key")
        );
    }
}
