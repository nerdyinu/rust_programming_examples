#[allow(dead_code)]
pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> Option<i32> {
    accounts
        .into_iter()
        .map(|account| account.iter().sum())
        .max()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let accounts = vec![vec![1, 2, 3], vec![3, 2, 1]];
        let res = maximum_wealth(accounts);
        assert_eq!(res, Some(6));
    }
}
