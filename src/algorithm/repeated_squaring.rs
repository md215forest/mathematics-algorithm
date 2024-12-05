const MOD: usize = 1000000007;

pub fn _run(a: usize, b: usize) -> usize {
    let mut result = 1;
    for _i in 0..b {
        println!("result: {}", result);
        result = result * a % MOD;
    }
    result
}

pub fn run2(mut a: usize, mut b: usize) -> usize {
    let mut result = 1;
    a = a % MOD;

    while b > 0 {
        if b % 2 == 1 {
            result = (result * a) % MOD;
        }
        b = b >> 1;
        a = (a * a) % MOD;
    }
    result
}
