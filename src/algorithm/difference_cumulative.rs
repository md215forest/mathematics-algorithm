pub fn caluculate(visitors: Vec<i32>) -> Vec<i32> {
    println!("{:?}", visitors);
    let mut cumulative_total = vec![0; visitors.len()];
    cumulative_total[0] = visitors[0];
    for i in 1..visitors.len() {
        cumulative_total[i] = cumulative_total[i - 1] + visitors[i];
    }
    cumulative_total
}
