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

pub fn climb_stairs(n: i32) -> i32 {
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
