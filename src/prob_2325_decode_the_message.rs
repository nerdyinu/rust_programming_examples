use std::collections::HashMap;
#[allow(dead_code)]
pub fn decode_message(key: &str, message: &str) -> Result<String, &'static str> {
    if key.is_empty() {
        return Err("Key cannot be empty");
    }
    if message.is_empty() {
        return Err("Message cannot be empty");
    }

    let mut substitution_map = HashMap::new();
    let mut next_char = b'a';

    key.chars().for_each(|ch| {
        if ch != ' ' && !substitution_map.contains_key(&ch) && next_char.is_ascii_lowercase() {
            substitution_map.insert(ch, char::from(next_char));
            next_char += 1;
        }
    });

    Ok(message
        .chars()
        .map(|c| *substitution_map.get(&c).unwrap_or(&' '))
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_message() {
        assert_eq!(
            decode_message(
                "the quick brown fox jumps over the lazy dog",
                "vkbs bs t suepuv"
            )
            .unwrap(),
            "this is a secret"
        );

        assert_eq!(
            decode_message("abcdefghijklmnopqrstuvwxyz", "hello world").unwrap(),
            "hello world"
        );

        assert_eq!(
            decode_message("", "hello world"),
            Err("Key cannot be empty")
        );

        assert_eq!(
            decode_message("the quick brown fox jumps over the lazy dog", ""),
            Err("Message cannot be empty")
        );
    }
}
