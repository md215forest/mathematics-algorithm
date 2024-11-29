pub fn sort(mut v: Vec<i32>) -> Vec<i32> {
    let stat = std::time::Instant::now();
    for i in 0..v.len() {
        let mut min = i;
        let mut min_value = v[i];
        for j in i..v.len() {
            if v[j] < min_value {
                min = j;
                min_value = v[j];
            }
        }
        v.swap(i, min);
    }
    let duration = stat.elapsed();
    println!("選択ソートの実行時間: {:?}", duration);
    v
}
