const RAND_NUM_ARRAY: [i32; 14] = [3, 1, 4, 1, 4, 65, 2, -31, -1, 99, 83, 782, 1, 0];

fn solve(l: i32, r: i32) -> i32 {
    println!("l: {}, r: {}", l, r);
    if r - l == 1 {
        println!("return: {}", RAND_NUM_ARRAY[l as usize]);
        return RAND_NUM_ARRAY[l as usize];
    }
    let m = (l + r) / 2;
    let s1 = solve(l, m);
    let s2 = solve(m, r);
    println!("s1: {}, s2: {}", s1, s2);
    s1 + s2
}

pub fn calculate(n: i32) -> i32 {
    solve(0, n)
}
