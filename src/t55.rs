use std::ops::ControlFlow;

struct Solution;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        nums.iter()
            .enumerate()
            .try_fold(0, |max_reachable, (i, &num)| {
                let num = num as usize;
                if i > max_reachable {
                    ControlFlow::Break(false)
                } else {
                    ControlFlow::Continue(std::cmp::max(max_reachable, i + num))
                }
            })
            .is_continue()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::can_jump(vec![2, 3, 1, 1, 4]));
    }

    #[test]
    fn test_2() {
        assert!(!Solution::can_jump(vec![3, 2, 1, 0, 4]));
    }
}
