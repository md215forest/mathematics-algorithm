mod algorithm;
fn main() {
    let count = algorithm::full_search::calculate(3, 4);
    println!("{}", count);
}
