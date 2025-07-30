use crate::Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut res = nums.clone();
        let mut acc = 1;
        for i in 0..nums.len() - 1 {
            acc *= nums[i];
            res[i + 1] = acc;
        }
        acc = 1;
        for i in (1..nums.len()).rev() {
            acc *= nums[i];
            res[i - 1] *= acc;
        }
        res
    }
}

#[test]
pub fn test_product_except_self_1() {
    assert_eq!(
        Solution::product_except_self(vec![1, 2, 3, 4]),
        vec![24, 12, 8, 6]
    );
}

#[test]
pub fn test_product_except_self_2() {
    assert_eq!(
        Solution::product_except_self(vec![-1, 1, 0, -3, 3]),
        vec![0, 0, 9, 0, 0]
    );
}
