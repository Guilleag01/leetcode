use crate::Solution;

use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        if s.is_empty() {
            return 0;
        }
        let mut letters: HashMap<char, usize> = HashMap::new();
        let mut cur_start: usize = 0;
        let mut max_length = 0;
        for (i, c) in s.chars().enumerate() {
            if max_length > i - cur_start && max_length >= s.len() - cur_start {
                return max_length as i32 + 1;
            }
            if let Some(idx) = letters.get(&c)
                && *idx >= cur_start
            {
                cur_start = idx + 1;
                *letters.get_mut(&c).unwrap() = i;
            } else {
                letters.insert(c, i);
                if (i - cur_start) > max_length {
                    max_length = i - cur_start;
                }
            }
        }
        max_length as i32 + 1
    }
}

// println!("{cur_start} {i} {max_length} {c} {letters:?}");
#[test]
pub fn test_length_of_longest_substring_1() {
    assert_eq!(
        Solution::length_of_longest_substring("abcabcbb".to_string()),
        3
    );
}

#[test]
pub fn test_length_of_longest_substring_2() {
    assert_eq!(
        Solution::length_of_longest_substring("bbbbb".to_string()),
        1
    );
}

#[test]
pub fn test_length_of_longest_substring_3() {
    assert_eq!(
        Solution::length_of_longest_substring("pwwkew".to_string()),
        3
    );
}

#[test]
pub fn test_length_of_longest_substring_4() {
    assert_eq!(Solution::length_of_longest_substring("au".to_string()), 2);
}

#[test]
pub fn test_length_of_longest_substring_5() {
    assert_eq!(Solution::length_of_longest_substring("dvdf".to_string()), 3);
}

#[test]
pub fn test_length_of_longest_substring_6() {
    assert_eq!(
        Solution::length_of_longest_substring("tmmzuxt".to_string()),
        5
    );
}
