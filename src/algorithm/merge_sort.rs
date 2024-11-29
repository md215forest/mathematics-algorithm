pub fn sort(a_array: Vec<i32>, b_array: Vec<i32>) -> Vec<i32> {
    let mut a_index = 0;
    let mut b_index = 0;
    let mut result = Vec::new();
    while a_index < a_array.len() && b_index < b_array.len() {
        if a_array[a_index] < b_array[b_index] {
            result.push(a_array[a_index]);
            a_index += 1;
        } else {
            result.push(b_array[b_index]);
            b_index += 1;
        }
    }
    result
}
