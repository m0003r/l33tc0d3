use std::ops::ControlFlow;

struct Solution;

impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let citation_counts =
            citations
                .iter()
                .fold(vec![0; citations.len() + 1], |mut acc, &citation| {
                    let idx = (citation as usize).min(citations.len());
                    acc[idx] += 1;
                    acc
                });

        let result = citation_counts.iter().enumerate().rev().try_fold(
            0,
            |greater_than_curr, (i, &count)| {
                let greater_than_curr = greater_than_curr + count;
                if greater_than_curr >= i {
                    ControlFlow::Break(i)
                } else {
                    ControlFlow::Continue(greater_than_curr)
                }
            },
        );

        match result {
            ControlFlow::Break(i) => i as i32,
            ControlFlow::Continue(_) => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::h_index(vec![3, 0, 6, 1, 5]), 3);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::h_index(vec![1, 3, 1]), 1);
    }
}
