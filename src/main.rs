mod algorithm;
#[allow(dead_code)]
fn main() {
    // let count = algorithm::full_search::calculate(3, 4);
    // println!("条件に該当する数は{}", count);

    // let index = algorithm::binary_search::search("bobby");
    // println!("単語の位置は{}", index);

    // let has_answer = algorithm::bit_full_serach::has_answer();
    // println!(
    //     "条件に該当する数があるかどうか{}",
    //     if has_answer { "Yes" } else { "No" }
    // );

    // let prime_vec = algorithm::primality_test::prime_factorization(20211225);
    // println!("素因数のリスト: {:?}", prime_vec);

    // let great_common_divisor = algorithm::euclidean_algorithm::calculate(123, 246);
    // println!("最大公約数: {}", great_common_divisor);

    let pi = algorithm::monte_carlo::calculate();
    println!("円周率の近似値: {}", pi);
}
