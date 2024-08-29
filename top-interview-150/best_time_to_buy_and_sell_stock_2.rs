fn main() {
    let prices = vec![7, 1, 5, 3, 6, 4];
    let profit = max_profit(prices);
    println!("profit: {}", profit);
}

pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut profit = 0;
    let mut buy_price = prices[0];
    for i in 1..prices.len() {
        let temp_profit = prices[i] - buy_price;
        if temp_profit > 0 {
            profit += temp_profit;
        }
        buy_price = prices[i];
    }
    profit
}
