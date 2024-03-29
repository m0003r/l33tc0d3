struct Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let m = m as usize;
        let n = n as usize;

        assert_eq!(nums1.len(), m + n);
        assert_eq!(nums2.len(), n);

        let mut m_iter = m;
        let mut n_iter = n;

        for t in (0..(m + n)).rev() {
            let next_m = nums1.get(m_iter - 1);
            let next_n = nums2.get(n_iter - 1);

            match (next_m, next_n) {
                (Some(m_val), Some(n_val)) => {
                    if m_val > n_val {
                        nums1[t] = *m_val;
                        m_iter -= 1;
                    } else {
                        nums1[t] = *n_val;
                        n_iter -= 1;
                    }
                }
                (Some(m_val), None) => {
                    nums1[t] = *m_val;
                    m_iter -= 1;
                }
                (None, Some(n_val)) => {
                    nums1[t] = *n_val;
                    n_iter -= 1;
                }
                (None, None) => unreachable!(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let m = 3;
        let mut nums2 = vec![2, 5, 6];
        let n = 3;
        Solution::merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);
    }

    #[test]
    fn test_2() {
        let mut nums1 = vec![1];
        let m = 1;
        let mut nums2 = vec![];
        let n = 0;
        Solution::merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, vec![1]);
    }

    #[test]
    fn test_3() {
        let mut nums1 = vec![0];
        let m = 0;
        let mut nums2 = vec![1];
        let n = 1;
        Solution::merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, vec![1]);
    }
}
