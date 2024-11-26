fn is_prime(n: i64) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..=((n as f64).sqrt() as i64) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

pub fn prime_factorization(n: i64) -> Vec<i64> {
    let mut n = n;
    let mut result = Vec::new();
    for i in 2..=((n as f64).sqrt() as i64) {
        while n % i == 0 {
            if is_prime(i) {
                result.push(i);
            }
            n /= i;
        }
    }
    if n != 1 {
        result.push(n);
    }
    result
}
