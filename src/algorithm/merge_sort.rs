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
    while a_index < a_array.len() {
        result.push(a_array[a_index]);
        a_index += 1;
    }
    while b_index < b_array.len() {
        result.push(b_array[b_index]);
        b_index += 1;
    }
    result
}

pub fn run(array: Vec<i32>) -> Vec<i32> {
    merge_sort(0, array.len() - 1, array)
}

fn merge_sort(l: usize, r: usize, array: Vec<i32>) -> Vec<i32> {
    let array_len = array.len();
    if array_len <= 1 {
        return array;
    }
    let m = array_len / 2;
    let left_array = merge_sort(l, m, array[..m].to_vec());
    let right_array = merge_sort(m, r, array[m..].to_vec());
    sort(left_array, right_array)
}
