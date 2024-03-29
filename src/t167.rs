struct Solution;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut left_iter = numbers.iter().enumerate().peekable();
        let mut right_iter = numbers.iter().enumerate().rev().peekable();

        while let (Some(&(li, left)), Some(&(ri, right))) = (left_iter.peek(), right_iter.peek()) {
            match (left + right).cmp(&target) {
                std::cmp::Ordering::Less => {
                    left_iter.next();
                }
                std::cmp::Ordering::Equal => {
                    return vec![li as i32 + 1, ri as i32 + 1];
                }
                std::cmp::Ordering::Greater => {
                    right_iter.next();
                }
            }
        }

        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![1, 2]);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::two_sum(vec![2, 3, 4], 6), vec![1, 3]);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::two_sum(vec![-1, 0], -1), vec![1, 2]);
    }
}
