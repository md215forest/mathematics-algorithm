use std::cmp::{max, min};

pub fn run(n: i32, k: i32) -> i32 {
    // 事象Bの個数 yojishou を数える
    let mut yojishou: i64 = 0;
    for a in 1..=n {
        for b in max(1, a - (k - 1))..=min(n, a + (k - 1)) {
            for c in max(1, a - (k - 1))..=min(n, a + (k - 1)) {
                if (b - c).abs() <= k - 1 {
                    yojishou += 1;
                }
            }
        }
    }
    (n * n * n) - yojishou as i32
}
