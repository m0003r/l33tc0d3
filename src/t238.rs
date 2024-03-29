struct Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut output = [1]
            .into_iter()
            .chain(nums[..nums.len() - 1].iter().scan(1, |acc, &x| {
                *acc *= x;
                Some(*acc)
            }))
            .collect::<Vec<i32>>();

        let mut right_part = 1;
        for i in (0..nums.len()).rev() {
            output[i] *= right_part;
            right_part *= nums[i];
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::product_except_self(vec![1, 2, 3, 4]),
            vec![24, 12, 8, 6]
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::product_except_self(vec![-1, 1, 0, -3, 3]),
            vec![0, 0, 9, 0, 0]
        );
    }
}
