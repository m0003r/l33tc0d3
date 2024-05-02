struct Solution;

impl Solution {

    pub fn next_permutation(nums: &mut Vec<i32>) {
        let decreasing_from = {
            let mut j = nums.len() - 1;
            while j > 0 && nums[j] <= nums[j - 1] {
                j -= 1;
            }
            j
        };

        eprintln!("In array {:?} decreasing_from is {}", nums, decreasing_from);

        nums[decreasing_from..].reverse();

        if decreasing_from == 0 {
            return;
        }

        let swap_with = {
            let mut j = decreasing_from;
            while nums[j] <= nums[decreasing_from - 1] {
                j += 1;
            }
            j
        };

        nums.swap(decreasing_from - 1, swap_with);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut nums = vec![1, 2, 3];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![1, 3, 2]);
    }

    #[test]
    fn test_2() {
        let mut nums = vec![3, 2, 1];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![1, 2, 3]);
    }

    #[test]
    fn test_3() {
        let mut nums = vec![1, 1, 5];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![1, 5, 1]);
    }

    #[test]
    fn test_4() {
        let mut nums = vec![1];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![1]);
    }

    #[test]
    fn test_5() {
        let mut nums = vec![5, 2, 6, 4, 3, 1];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![5, 3, 1, 2, 4, 6]);
    }
}