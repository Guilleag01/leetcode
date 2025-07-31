use crate::Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        Solution::add_two_numbers_rec(l1, l2, 0)
    }

    pub fn add_two_numbers_rec(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
        carry: i32,
    ) -> Option<Box<ListNode>> {
        Option::Some(Box::new(match (l1, l2) {
            (None, None) => {
                if carry > 0 {
                    ListNode {
                        val: 1,
                        next: Option::None,
                    }
                } else {
                    return Option::None;
                }
            }
            (None, Some(a)) => {
                let mut sum = a.val + carry;
                let mut new_carry = 0;
                if sum > 9 {
                    new_carry = 1;
                    sum -= 10;
                }
                ListNode {
                    val: sum,
                    next: Solution::add_two_numbers_rec(Option::None, a.next, new_carry),
                }
            }
            (Some(a), None) => {
                let mut sum = a.val + carry;
                let mut new_carry = 0;
                if sum > 9 {
                    new_carry = 1;
                    sum -= 10;
                }
                ListNode {
                    val: sum,
                    next: Solution::add_two_numbers_rec(a.next, Option::None, new_carry),
                }
            }
            (Some(a), Some(b)) => {
                let mut sum = a.val + b.val + carry;
                let mut new_carry = 0;
                if sum > 9 {
                    new_carry = 1;
                    sum -= 10;
                }
                ListNode {
                    val: sum,
                    next: Solution::add_two_numbers_rec(a.next, b.next, new_carry),
                }
            }
        }))
    }
}

#[test]
pub fn test_add_two_numbers_1() {
    assert_eq!(
        i32_from_list_node(Solution::add_two_numbers(
            list_node_from_array(&[2, 4, 3]),
            list_node_from_array(&[5, 6, 4])
        )),
        807
    );
}

#[test]
pub fn test_add_two_numbers_2() {
    assert_eq!(
        i32_from_list_node(Solution::add_two_numbers(
            list_node_from_array(&[0]),
            list_node_from_array(&[0])
        )),
        0
    );
}

#[test]
pub fn test_add_two_numbers_3() {
    assert_eq!(
        i32_from_list_node(Solution::add_two_numbers(
            list_node_from_array(&[9, 9, 9, 9, 9, 9, 9]),
            list_node_from_array(&[9, 9, 9, 9])
        )),
        10009998
    );
}

pub fn list_node_from_array(arr: &[i32]) -> Option<Box<ListNode>> {
    Option::Some(Box::new(ListNode {
        val: arr[0],
        next: if arr.len() > 1 {
            list_node_from_array(&arr[1..])
        } else {
            Option::None
        },
    }))
}

pub fn i32_from_list_node(l: Option<Box<ListNode>>) -> i32 {
    if let Some(a) = l {
        a.val + i32_from_list_node(a.next) * 10
    } else {
        0
    }
}
