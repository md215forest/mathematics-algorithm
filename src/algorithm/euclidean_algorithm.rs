use std::cmp::{max, min};

pub fn calculate(n: i32, m: i32) -> i32 {
    let mut larger = max(n, m);
    let mut smaller = min(n, m);
    while larger >= 1 && smaller >= 1 {
        if smaller < larger {
            larger = larger % smaller;
        } else {
            smaller = smaller % larger;
        }
    }
    if smaller >= 1 {
        return smaller;
    }
    larger
}
