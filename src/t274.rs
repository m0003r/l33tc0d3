struct Solution;

impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let citation_counts = citations.iter().
            fold(vec![0; citations.len() + 1], |mut acc, &citation| {
                let idx = (citation as usize).min(citations.len());
                acc[idx] += 1;
                acc
        });

        citation_counts.iter()
            .enumerate()
            .take_while(|&(idx, &count)| idx + count <= citations.len())
            .last()
            .map(|(idx, _)| idx as i32)
            .unwrap_or(0)
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