struct Solution;
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut good = 0;
        for curr in 0..nums.len() {
            if nums[curr] != val {
                nums[good] = nums[curr];
                good += 1;
            }
        }
        good as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut nums = vec![3, 2, 2, 3];
        assert_eq!(Solution::remove_element(&mut nums, 3), 2);
        assert_eq!(nums[..2], vec![2, 2]);
    }

    #[test]
    fn test_2() {
        let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
        assert_eq!(Solution::remove_element(&mut nums, 2), 5);
        assert_eq!(nums[..5], vec![0, 1, 3, 0, 4]);
    }
}
