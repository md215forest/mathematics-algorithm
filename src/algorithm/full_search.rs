/**
 * Full search algorithm
 * 下記の条件の時、1以上N以下の2つの数の組み合わせで、和がS以下になる組み合わせの数を求める
 * 1 ≦ N ≦ 1000,　1 ≦ S ≦ 2000
 */
pub fn calculate(n: i32, s: i32) -> i32 {
    let mut count = 0;
    for i in 1..=n {
        for j in 1..=n {
            if i + j <= s {
                count += 1;
            }
        }
    }
    count
}
