struct Solution;

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let nums = nums.as_mut_slice();

        if k == 0 || nums.len() < 2 {
            return;
        }

        assert!(k >= 0, "k must be non-negative");
        let k = (k as usize) % nums.len();

        // we will rotate array backwards
        let rk = (nums.len() - k) % nums.len();
        let mut remains = nums.len();
        let mut start = 0;

        while remains > 0 {
            let last_element = nums[start];
            let mut curr = start;
            loop {
                let next = (curr + rk) % nums.len();
                nums[curr] = nums[next];
                curr = next;
                remains -= 1;
                if curr == start {
                    nums[(start + k) % nums.len()] = last_element;
                    break;
                }
            }
            start += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
        Solution::rotate(&mut nums, 3);
        assert_eq!(nums, vec![5, 6, 7, 1, 2, 3, 4]);
    }

    #[test]
    fn test_2() {
        let mut nums = vec![1, 2, 3, 4];
        Solution::rotate(&mut nums, 2);
        assert_eq!(nums, vec![3, 4, 1, 2]);
    }

    #[test]
    fn test_31() {
        let mut nums = vec![1, 2, 3];
        Solution::rotate(&mut nums, 4);
        assert_eq!(nums, vec![3, 1, 2]);
    }
}
