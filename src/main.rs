use rand::Rng;

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

    // let _primes = algorithm::primality_test::get_primes(1000000);
    // println!("素数のリスト: {:?}", primes);

    // let great_common_divisor = algorithm::euclidean_algorithm::calculate(123, 241);
    // println!("最大公約数: {}", great_common_divisor);

    // let pi = algorithm::monte_carlo::calculate();
    // println!("円周率の近似値: {}", pi);

    // let _sorted_vec = algorithm::sort::sort(get_array(10000, 10001));
    // println!("ソート後の配列: {:?}", sorted_vec);

    // let sum = algorithm::devide_and_conquer::calculate(4);
    // println!("リストの1個目から4個目までの和: {}", sum);

    // let result = algorithm::merge_sort::sort(vec![13, 34, 50, 75], vec![11, 20, 28, 62]);
    // println!("マージ後の配列: {:?}", result);

    // let _result = algorithm::merge_sort::run(get_array(10000, 10001));
    // println!("マージソート後の配列: {:?}", result);

    // let min_step = algorithm::dynamic_programming::frog_movement(vec![8, 6, 9, 2, 1]);
    // println!("最小の移動回数: {}", min_step);

    // let ways = algorithm::dynamic_programming::climb_stairs(30);
    // println!("登る方法の数: {}", ways);

    // let max_value = algorithm::dynamic_programming::knapsack(10, get_items(4));
    // println!("最大の価値: {}", max_value);

    // let cumulative_total = algorithm::difference_cumulative::caluculate(get_array(10, 10));
    // println!("累積和: {:?}", cumulative_total);

    // let square_root = algorithm::newton_method::square_root(2.0);
    // println!("平方根: {}", square_root);

    // let _primes = algorithm::sieve_of_erastosthenes::get_primes(1000000);
    // println!("素数のリスト: {:?}", primes);

    // let connected = algorithm::depth_first_search::run();
    // println!("連結しているか: {}", connected);

    // let connected = algorithm::breadth_first_search::run();
    // println!("連結しているか: {}", connected);

    // let remainder = algorithm::repeated_squaring::run2(3, 40);
    // println!("余り: {}", remainder);

    let fibonacci = algorithm::fibonacci::run(10);
    println!("フィボナッチ数列: {:?}", fibonacci);
}

fn _get_array(n: i32, max: i32) -> Vec<i32> {
    let mut array = Vec::new();
    for _i in 0..n {
        let mut rng = rand::thread_rng();
        let num = rng.gen_range(1..max);
        array.push(num);
    }
    array
}

// fn get_items(n: i32) -> Vec<algorithm::dynamic_programming::Item> {
//     let mut items = Vec::new();
//     for _i in 0..n {
//         let mut rng = rand::thread_rng();
//         let weight = rng.gen_range(1..10);
//         let value = rng.gen_range(1..10) * 10;
//         let item = algorithm::dynamic_programming::Item::new(weight as usize, value as usize);
//         items.push(item);
//     }
//     items
// }
