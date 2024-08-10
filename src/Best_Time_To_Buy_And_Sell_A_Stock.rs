/*
You are given an array prices where prices[i] is the price of a
given stock on the [i]th day.

You want to maximize your profit by choosing a single day to buy
one stock and choosing a different day in the future to sell that stock.

Return the maximum profit you can achieve from this transaction.
If you cannot achieve any profit, return 0.
*/
//Time complexity: O(n)
fn max_profit(prices: Vec<i32>) -> i32
{
    if prices.is_empty()
    {
        return 0;
    }

    let mut min_price = prices[0];
    let mut max_profit = 0;

    for price in prices.iter().skip(1)
    {
        if *price < min_price
        {
            min_price = *price;
        }
        else
        {
            let potential_profit = price - min_price;
            if potential_profit > max_profit
            {
                max_profit = potential_profit;
            }
        }
    }
    max_profit
}

fn main()
{
    let nums: Vec<i32> = vec![7,1,5,3,6,4];
    let result = max_profit(nums);
    println!("Result: {}", result);
}