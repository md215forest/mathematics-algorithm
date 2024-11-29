pub fn _frog_movement(h: Vec<i32>) -> i32 {
    let mut dp = vec![0; h.len()];
    for i in 0..h.len() {
        if i == 0 {
            dp[i] = 0;
        }
        if i == 1 {
            dp[i] = (h[i] - h[i - 1]).abs();
        }
        if i >= 2 {
            let v1 = dp[i - 1] + (h[i] - h[i - 1]).abs();
            let v2 = dp[i - 2] + (h[i] - h[i - 2]).abs();
            dp[i] = std::cmp::min(v1, v2);
        }
    }
    dp[h.len() - 1]
}

pub fn _climb_stairs(n: i32) -> i32 {
    let mut dp = vec![0; (n + 1) as usize];
    for i in 0..=n {
        if i <= 1 {
            dp[i as usize] = 1;
        }
        if i >= 2 {
            dp[i as usize] = dp[(i - 1) as usize] + dp[(i - 2) as usize];
        }
    }
    dp[n as usize]
}

pub fn knapsack(max_weight: usize, items: Vec<Item>) -> usize {
    let n = items.len();
    let mut dp = vec![vec![0; max_weight + 1]; n + 1];
    for i in 1..=n {
        for w in 0..=max_weight {
            if items[i - 1].weight <= w {
                dp[i][w] =
                    dp[i - 1][w].max(dp[i - 1][(w) - items[i - 1].weight] + items[i - 1].value);
            } else {
                dp[i][w] = dp[i - 1][w];
            }
        }
    }
    dp[n][max_weight]
}

#[derive(Debug)]
pub struct Item {
    weight: usize,
    value: usize,
}

impl Item {
    pub fn new(weight: usize, value: usize) -> Self {
        Self { weight, value }
    }
}
