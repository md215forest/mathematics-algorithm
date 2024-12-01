pub fn square_root(n: f64) -> f64 {
    let mut a = n;
    for _i in 0..=5 {
        let x = a;
        let y = a * a;

        let line_a = 2.0 * x;
        let line_b = y - line_a * x;

        println!("x: {}, y: {}, a: {}", x, y, a);
        a = (n - line_b) / line_a;
    }
    a
}
