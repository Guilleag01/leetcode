use crate::Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for (i, e1) in nums.clone().iter().enumerate() {
            for (j, e2) in nums[i + 1..].iter().enumerate() {
                if e1 + e2 == target {
                    return vec![i as i32, (i + (j + 1)) as i32];
                }
            }
        }
        unreachable!() // there is exactly one solution
    }
}

#[test]
pub fn test_two_sum_1() {
    assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
}

#[test]
pub fn test_two_sum_2() {
    assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
}

#[test]
pub fn test_two_sum_3() {
    assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
}
