pub fn max_stock_profit(prices: Vec<i32>, opcount: usize) -> i32 {
    let mut loses = vec![i32::MIN; opcount];
    let mut profits = vec![0; opcount];

    for p in prices.iter() {
        print!("Price: {:2}, ", p);
        let mut prev_profit = 0;
        for o in 0..opcount {
            print!("{o}: ");
            let curr_loss = prev_profit - p;
            loses[o] = loses[o].max(curr_loss);
            
            print!("[{:3}, {:3}] ", curr_loss, loses[o]);

            prev_profit = profits[o];

            let prev_loss = loses[o];
            let curr_profit = prev_loss + p;
            profits[o] = profits[o].max(curr_profit);
            print!("[{:3}, {:3}] || ", curr_profit, profits[o]);
        }
        println!();
    }

    return profits[opcount-1];
}