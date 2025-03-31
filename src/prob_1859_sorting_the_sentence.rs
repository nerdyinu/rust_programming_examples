#![allow(dead_code)]
pub fn sort_sentence(s: &str) -> String {
    let mut words: Vec<&str> = s.split_whitespace().collect();

    words.sort_unstable_by_key(|ch| ch.chars().last().expect("guaranteed to be non-empty"));

    words
        .iter()
        .map(|word| &word[..word.len() - 1])
        .collect::<Vec<&str>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_sentence() {
        let s = "is2 sentence4 This1 a3".to_string();
        let expected = "This is a sentence".to_string();
        assert_eq!(sort_sentence(&s), expected);

        let s = "Myself2 Me1 I4 and3".to_string();
        let expected = "Me Myself and I".to_string();
        assert_eq!(sort_sentence(&s), expected);
    }
}
