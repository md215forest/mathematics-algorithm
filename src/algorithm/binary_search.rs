use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec;

#[derive(Clone)]
struct Word {
    word: String,
    index: usize,
}
/**
 * Binary search algorithm
 * words.txtからできるだけ少ない検索で位置を出力する
 */
pub fn search(target: &str) -> i32 {
    let path = Path::new("src/public/words.txt");
    let file = File::open(&path).expect("Could not open file");
    let reader = io::BufReader::new(file);

    let words: Vec<Word> = reader
        .lines()
        .enumerate()
        .map(|(index, line)| {
            let line = line.unwrap();
            Word {
                word: line.trim().to_string(),
                index: index + 1,
            }
        })
        .collect();

    search_word(target, words, 1)
}

fn search_word(target: &str, words: Vec<Word>, count: i32) -> i32 {
    let words_length = words.len();
    if words_length == 1 {
        println!("{}", words[0].word);
        return -1;
    }
    let mid = words.len() / 2;

    let word = &words[mid].word;

    if word == target {
        return words[mid].index as i32;
    }

    if is_apper(target, &words[mid]) {
        search_word(target, words[0..mid].to_vec(), count + 1)
    } else {
        search_word(target, words[mid..].to_vec(), count + 1)
    }
}

fn is_apper(target: &str, word: &Word) -> bool {
    let mut word_vec = vec![target, &word.word];
    word_vec.sort();
    word_vec[0] == target
}
