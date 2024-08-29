fn main() {
    let prices = vec![1, 2, 3, 4];
    let profit = max_profit(prices);
    println!("profit: {}", profit);
}

pub fn max_profit_bad_solution(prices: Vec<i32>) -> i32 {
    let mut profit = 0;
    for i in 0..prices.len() {
        for j in i + 1..prices.len() {
            let diff = prices[j] - prices[i];
            if diff > profit {
                profit = diff;
            }
        }
    }
    profit
}

pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut profit = 0;
    let mut buy_price = prices[0];
    for i in 1..prices.len() {
        let temp_profit = prices[i] - buy_price;
        if temp_profit > profit {
            profit = temp_profit;
        }
        if prices[i] < buy_price {
            buy_price = prices[i];
        }
    }
    profit
}
