use std::collections::HashMap;
#[allow(dead_code)]
pub fn decode_message(key: String, message: String) -> Result<String, &'static str> {
    if key.is_empty() {
        return Err("Key cannot be empty");
    }
    if message.is_empty() {
        return Err("Message cannot be empty");
    }

    let mut substitution_map = HashMap::new();
    let mut next_char = b'a';

    substitution_map.insert(' ', ' ');

    key.char_indices().for_each(|(i, ch)| {
        if ch != ' ' && !substitution_map.contains_key(&ch) && next_char <= b'z' {
            substitution_map.insert(ch, (next_char as char));
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
        let key = "the quick brown fox jumps over the lazy dog".to_string();
        let message = "vkbs bs t suepuv".to_string();
        assert_eq!(
            decode_message(key, message).unwrap(),
            "this is a secret".to_string()
        );

        let key = "abcdefghijklmnopqrstuvwxyz".to_string();
        let message = "hello world".to_string();
        assert_eq!(
            decode_message(key, message).unwrap(),
            "hello world".to_string()
        );

        let key = "".to_string();
        let message = "hello world".to_string();
        assert_eq!(decode_message(key, message), Err("Key cannot be empty"));

        let key = "the quick brown fox jumps over the lazy dog".to_string();
        let message = "".to_string();
        assert_eq!(decode_message(key, message), Err("Message cannot be empty"));
    }
}
