// fn fibonacci(n: usize) -> usize {
//     if n <= 1 {
//         n
//     } else {
//         fibonacci(n - 1) + fibonacci(n - 2)
//     }
// }

#[derive(Clone, Copy)]
struct Matrix {
    p: [[u64; 2]; 2],
}

impl Matrix {
    fn new() -> Self {
        Matrix { p: [[0; 2]; 2] }
    }

    fn init() -> Self {
        let mut init = Matrix::new();
        init.p[0][0] = 1;
        init.p[0][1] = 1;
        init.p[1][0] = 1;
        init.p[1][1] = 0;
        init
    }

    fn multiplication(a: Matrix, b: Matrix) -> Matrix {
        let mut c = Matrix::new();
        for i in 0..2 {
            for k in 0..2 {
                for j in 0..2 {
                    c.p[i][j] += a.p[i][k] * b.p[k][j];
                    c.p[i][j] %= 1_000_000_000;
                }
            }
        }
        c
    }

    fn power(mut a: Matrix, n: u64) -> Matrix {
        let mut q = Matrix::new();
        let mut flag = false;
        for i in 0..60 {
            if (n & (1 << i)) != 0 {
                if !flag {
                    q = a;
                    flag = true;
                } else {
                    q = Matrix::multiplication(q, a);
                }
            }
            a = Matrix::multiplication(a, a);
        }
        q
    }
}

pub fn run(n: usize) -> Vec<usize> {
    let start = std::time::Instant::now();
    let mut result = Vec::new();
    for i in 0..n {
        let a = Matrix::init();
        let q = Matrix::power(a, i as u64);
        result.push(q.p[0][1] as usize);
    }
    println!("処理時間: {:?}", start.elapsed());
    result
}
