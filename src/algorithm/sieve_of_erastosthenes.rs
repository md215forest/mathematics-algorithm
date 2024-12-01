pub fn get_primes(n: usize) -> Vec<usize> {
    let mut is_primes = vec![true; n + 1];
    let mut primes = Vec::new();
    let start = std::time::Instant::now();
    for i in 2..=n {
        if is_primes[i] {
            primes.push(i);
            let mut multiple = i * i;
            while multiple <= n {
                is_primes[multiple] = false;
                multiple += i;
            }
        }
    }
    let duration = start.elapsed();
    println!("エラトステネスのふるいの実行時間: {:?}", duration);
    primes
}
