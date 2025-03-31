#[allow(dead_code)]
pub fn defang_i_paddr(address: &str) -> String {
    address.replace('.', "[.]")
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_defang_i_paddr_with_multiple_dots() {
        let address = "255.100.50.0";
        let expected = "255[.]100[.]50[.]0".to_string();
        assert_eq!(defang_i_paddr(address), expected);
    }
}
