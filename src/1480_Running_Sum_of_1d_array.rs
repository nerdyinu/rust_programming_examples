impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut prefix = vec![0; nums.len() + 1];
        for i in 0..nums.len() as usize {
            prefix[i + 1] = prefix[i] + nums[i]
        }
        prefix.remove(0);
        return prefix;
    }
}
