mod algorithm;
fn main() {
    let count = algorithm::full_search::calculate(3, 4);
    println!("条件に該当する数は{}", count);

    let index = algorithm::binary_search::search("bobby");
    println!("単語の位置は{}", index);

    algorithm::bit_full_serach::calculate();
}
