pub fn calculate(mut n: i32, mut m: i32) -> i32 {
    while n >= 1 && m >= 1 {
        if n < m {
            m = m % n
        } else {
            n = n % m
        }
    }
    if n >= 1 {
        return n;
    }
    m
}
