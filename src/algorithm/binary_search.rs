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
pub fn search(word: &str) -> i32 {
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

    search_word(word, words, 1)
}

fn search_word(word: &str, words: Vec<Word>, count: i32) -> i32 {
    let mid = words.len() / 2;

    if words[mid].word == word {
        return words[mid].index as i32;
    }

    let mut word_vec = vec![word, &words[mid].word];
    word_vec.sort();
    let is_apper = word_vec[0] == word;
    if is_apper {
        search_word(word, words[0..mid].to_vec(), count + 1)
    } else {
        search_word(word, words[mid..].to_vec(), count + 1)
    }
}
