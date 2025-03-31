#![allow(dead_code)]
pub fn most_words_found(sentences: &[String]) -> Result<usize, &'static str> {
    if sentences.is_empty() {
        return Err("Input sentences cannot be empty");
    }
    Ok(sentences
        .iter()
        .map(|sentence| sentence.split_ascii_whitespace().count())
        .max()
        .expect("guaranteed to be non-empty"))
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_most_words_found() {
        let sentences = &[
            "alice and bob love leetcode".to_string(),
            "i think so too".to_string(),
            "this is great thanks very much".to_string(),
        ];
        assert_eq!(most_words_found(sentences), Ok(6));
        let sentences = &[
            "please wait".to_string(),
            "continue to fight".to_string(),
            "continue to win".to_string(),
        ];
        assert_eq!(most_words_found(sentences), Ok(3));
    }
}
