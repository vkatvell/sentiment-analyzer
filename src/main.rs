use regex::Regex;
use rustc_hash::FxHashMap;
use rustc_hash::FxHashSet;
use serde::Deserialize;
use std::error::Error;
use std::time::Instant;
use stop_words::{get, LANGUAGE};
use unicode_segmentation::UnicodeSegmentation;
#[derive(Debug, Deserialize)]
struct Tweet {
    sentiment: u8,
    id: u64,
    date: String,
    query: String,
    user: String,
    tweet: String,
}

fn get_stop_words() -> Vec<String> {
    get(LANGUAGE::English)
        .iter()
        .map(|w| w.replace('\"', ""))
        .collect()
}

fn process_word(w: &str, special_char_regex: &Regex) -> Option<String> {
    let stopwords: FxHashSet<String> = get_stop_words()
        .iter()
        .map(|s| s.to_owned())
        .collect::<FxHashSet<String>>();

    let punc_vec = vec![
        "!", "\"", "#", "$", "%", "&", "'", "(", ")", "*", "+", ",", ";", ".", "/", ":", ",", "<",
        "=", ">", "?", "@", "[", "\\", "]", "^", "_", "`", "{", "|", "}", "~", "-",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect::<Vec<String>>();

    let punctuation: FxHashSet<String> = punc_vec
        .iter()
        .map(|s| s.to_string())
        .collect::<FxHashSet<String>>();

    let word = special_char_regex.replace_all(w.trim(), "").to_lowercase();

    if word.is_empty()
        || (word.graphemes(true).count() == 1) && punctuation.contains(&word)
        || stopwords.contains(&word)
    {
        return None;
    }

    Some(word)
}

fn get_special_char_regex() -> Regex {
    Regex::new(r"('s|,|\.)").unwrap()
}

fn build_word_score_map(
    path: &str,
    mut wordmap: FxHashMap<String, f32>,
) -> Result<(), Box<dyn Error>> {
    // Reading from csv path
    let mut reader = csv::Reader::from_path(path)?;

    // Count records
    let mut count: i32 = 0;

    // Iterating through records and deserializing into Tweet struct
    for result in reader.deserialize() {
        let record: Tweet = result?;

        // // Tokenizing tweets
        // let _collection: Vec<&str> = record.tweet.split_whitespace().collect();

        // Tokenizing tweets and processing each word to remove punctuation
        let collect: FxHashSet<String> = record
            .tweet
            .split_word_bounds()
            .filter_map(|w| process_word(w, &get_special_char_regex()))
            .collect();

        // Scoring words higher if pos. sentiment
        for word in collect {
            if record.sentiment == 4 {
                *wordmap.entry(word.to_owned()).or_insert(0.0) += 5.0;
            }
            *wordmap.entry(word).or_insert(0.0) -= 1.0;
        }

        // Discarding unused struct values
        let _ = record.query;
        let _ = record.date;
        let _ = record.user;
        let _ = record.id;
        let _ = record.sentiment;

        count += 1;
    }

    // println!("\n\nWords and their score: {:?}", wordmap);

    let mapcount = wordmap.len();
    println!("\n\nLength of wordmap: {}", mapcount);

    println!("\nRead {} records", count);

    Ok(())
}

fn main() {
    // Creating hashmap for words and their score
    let word_map: FxHashMap<String, f32> = FxHashMap::default();

    let start_time = Instant::now();
    if let Err(e) = build_word_score_map("./sent_analysis_data/train_dataset_10val.csv", word_map) {
        eprintln!("{}", e);
    }
    let elapsed_time = start_time.elapsed();

    println!("\nTime: {:?}", elapsed_time);
}
