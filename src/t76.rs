struct Solution;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let s = s.as_bytes();
        let t = t.as_bytes();

        let mut total_stat = -(t.len() as i32);
        let mut stats = t.iter().fold([0; 128], |mut acc, c| {
            let c = *c as usize;
            acc[c] -= 1;
            acc
        });

        let mut start = 0;
        let mut end = 0;
        let mut min_region = None;

        while total_stat >= 0 || end < s.len() {
            if total_stat >= 0 {
                let c = s[start] as usize;
                if stats[c] == 0 {
                    total_stat -= 1;
                }
                stats[c] -= 1;
                start += 1;
            } else {
                let c = s[end] as usize;
                stats[c] += 1;
                if stats[c] <= 0 {
                    total_stat += 1;
                }
                end += 1;
            }

            if total_stat == 0 {
                min_region = match min_region {
                    Some((min_start, min_end)) => {
                        if min_end - min_start > end - start {
                            Some((start, end))
                        } else {
                            Some((min_start, min_end))
                        }
                    }
                    None => Some((start, end)),
                };
            }
        }

        match min_region {
            Some((min_start, min_end)) => {
                std::str::from_utf8 (&s[min_start..min_end]).unwrap().to_string()
            }
            None => "".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::min_window("ADOBECODEBANC".to_string(), "ABC".to_string()),
            "BANC".to_string()
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::min_window("a".to_string(), "a".to_string()),
            "a".to_string()
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::min_window("a".to_string(), "aa".to_string()),
            "".to_string()
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(
            Solution::min_window("aa".to_string(), "aa".to_string()),
            "aa".to_string()
        );
    }
}