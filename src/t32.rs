struct Solution;

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut stack = Vec::new();
        let mut max_len = 0;
        let mut last_start = 0;

        for (i, c) in s.chars().enumerate() {
            match c {
                '(' => {
                    stack.push(i)
                }
                ')' => {
                    let last_was = stack.pop();
                    match last_was {
                        None => {
                            last_start = i + 1;
                        }
                        Some(pos) => {
                            match stack.last() {
                                None => {
                                    max_len = max_len.max(i - last_start + 1);
                                }
                                Some(v) => {
                                    max_len = max_len.max(i - v);
                                }
                            }
                        }
                    }
                }
                _ => unreachable!()
            }
        }

        max_len as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_valid_parentheses() {
        assert_eq!(Solution::longest_valid_parentheses("(()".to_string()), 2);
        assert_eq!(Solution::longest_valid_parentheses(")()())".to_string()), 4);
        assert_eq!(Solution::longest_valid_parentheses("".to_string()), 0);
        assert_eq!(Solution::longest_valid_parentheses("(".to_string()), 0);
        assert_eq!(Solution::longest_valid_parentheses(")".to_string()), 0);
        assert_eq!(Solution::longest_valid_parentheses("()".to_string()), 2);
        assert_eq!(Solution::longest_valid_parentheses("()(()".to_string()), 2);
        assert_eq!(Solution::longest_valid_parentheses("()()".to_string()), 4);
        assert_eq!(Solution::longest_valid_parentheses("()())((())))()()()()))()()(".to_string()), 8);
    }
}
