struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min_price = prices[0];
        let mut max_profit_fst = 0;
        // it is profits from first transaction
        let profits_from_first = prices
            .iter()
            .map(|&price| {
                min_price = min_price.min(price);
                max_profit_fst = max_profit_fst.max(price - min_price);
                max_profit_fst
            })
            .collect::<Vec<_>>();

        // it is profits from second transaction and maximum overall profit
        let mut max_price = prices[prices.len() - 1];
        let mut max_profit_snd = 0;
        let mut max_profit = 0;
        for (idx, price) in prices.iter().enumerate().rev() {
            max_price = max_price.max(*price);
            max_profit_snd = max_profit_snd.max(max_price - price);
            max_profit = max_profit.max(profits_from_first[idx] + max_profit_snd);
        }

        max_profit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 7);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5]), 4);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
    }

    #[test]
    fn test_4() {
        assert_eq!(Solution::max_profit(vec![1, 2, 4, 2, 5, 7, 2, 4, 9, 0]), 13);
    }

    #[test]
    fn test_202() {
        assert_eq!(
            Solution::max_profit(vec![5, 2, 3, 2, 6, 6, 2, 9, 1, 0, 7, 4, 5, 0]),
            14
        );
    }
}
