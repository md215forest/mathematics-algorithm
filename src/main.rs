mod algorithm;
fn main() {
    let count = algorithm::full_search::calculate(3, 4);
    println!("{}", count);

    let index = algorithm::binary_search::search("bobby");
    println!("{}", index);
}
