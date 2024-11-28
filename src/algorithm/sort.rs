pub fn sort(mut v: Vec<i32>) -> Vec<i32> {
    for i in 0..v.len() {
        let mut min = 0;
        let mut min_value = 0;
        for j in i..v.len() {
            if j == i {
                min = j;
                min_value = v[j];
            } else {
                if v[j] < min_value {
                    min = j;
                    min_value = v[j];
                }
            }
        }
        v.swap(i, min);
    }
    v
}
