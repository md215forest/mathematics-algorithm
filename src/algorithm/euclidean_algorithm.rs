pub fn calculate(n: i32, m: i32) -> i32 {
    if m == 0 {
        return n;
    }
    return calculate(m, n % m);
}
