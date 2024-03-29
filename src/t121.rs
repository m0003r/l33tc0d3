struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        assert!(prices.len() > 0);
        prices
            .iter()
            .fold((prices[0], 0), |(min_price, curr_profit), price| {
                (min_price.min(*price), curr_profit.max(price - min_price))
            })
            .1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
    }
}
