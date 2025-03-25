impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        let mut max_wealth = 0;
        for account in accounts {
            let sum: i32 = account.iter().sum();
            if sum > max_wealth {
                max_wealth = sum;
            }
        }
        max_wealth
    }
}
