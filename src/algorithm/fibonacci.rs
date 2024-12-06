fn fibonacci(n: usize) -> usize {
    if n <= 1 {
        n
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

pub fn run(n: usize) -> Vec<usize> {
    let start = std::time::Instant::now();
    let mut result = Vec::new();
    for i in 0..n {
        print!("i: {} ", i);
        result.push(fibonacci(i));
    }
    println!("処理時間: {:?}", start.elapsed());
    result
}
