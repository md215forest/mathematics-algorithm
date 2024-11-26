pub fn has_answer() -> bool {
    let n: usize = 3;
    let s: i64 = 11;

    let a = [2, 5, 9];

    for i in 0..=(1 << n) {
        let mut sum = 0;
        for j in 0..n {
            if i & (1 << j) != 0 {
                sum += a[j];
            }
        }
        if sum == s {
            return true;
        }
    }
    false
}
