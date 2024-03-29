struct Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let m = m as usize;
        let n = n as usize;

        assert_eq!(nums1.len(), m + n);
        assert_eq!(nums2.len(), n);

        unsafe {
            let mut m_iter = std::slice::from_raw_parts(nums1.as_ptr(), m).iter().rev().peekable();
            let mut n_iter = nums2.iter().rev().peekable();

            nums1.iter_mut().rev().for_each(|num| {
                let m = m_iter.peek();
                let n = n_iter.peek();

                match (m, n) {
                    (Some(&m), Some(&n)) => {
                        if m > n {
                            *num = *m;
                            m_iter.next();
                        } else {
                            *num = *n;
                            n_iter.next();
                        }
                    }
                    (Some(&m), None) => {
                        *num = *m;
                        m_iter.next();
                    }
                    (None, Some(&n)) => {
                        *num = *n;
                        n_iter.next();
                    }
                    (None, None) => {
                        unreachable!()
                    }
                }
            });
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
