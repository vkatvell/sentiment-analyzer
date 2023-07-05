use regex::Regex;
use rustc_hash::FxHashMap;
use rustc_hash::FxHashSet;
use serde::Deserialize;
use std::error::Error;
use std::time::Instant;
use stop_words::{get, LANGUAGE};
use unicode_segmentation::UnicodeSegmentation;
#[derive(Debug, Deserialize)]
struct TrainTweet {
    sentiment: u8,
    id: u64,
    date: String,
    query: String,
    user: String,
    tweet: String,
}
#[derive(Debug, Deserialize)]
struct TestTweet {
    id: u64,
    date: String,
    query: String,
    user: String,
    tweet: String,
}

#[derive(Debug, Deserialize)]
struct TestSentID {
    sentiment: u8,
    id: u64,
}

// Getting stop words and collecting into HashSet
fn get_stop_words() -> FxHashSet<String> {
    get(LANGUAGE::English)
        .iter()
        .map(|w| w.replace('\"', ""))
        .collect::<Vec<String>>()
        .iter()
        .map(|s| s.to_owned())
        .collect::<FxHashSet<String>>()
}

// Processing words and removing stop words, special chars, and punctuation
fn process_word(
    w: &str,
    special_char_regex: &Regex,
    stopwords: &FxHashSet<String>,
) -> Option<String> {
    let punc_vec: Vec<String> = vec![
        "!", "\"", "#", "$", "%", "&", "'", "(", ")", "*", "+", ",", ";", ".", "/", ":", ",", "<",
        "=", ">", "?", "@", "[", "\\", "]", "^", "_", "`", "{", "|", "}", "~", "-",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();

    let punctuation: FxHashSet<String> = punc_vec.iter().map(|s| s.to_string()).collect();

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

// Building word score map
fn build_word_score_map(path: &str) -> Result<FxHashMap<String, f32>, Box<dyn Error>> {
    // Reading from csv path
    let mut reader = csv::Reader::from_path(path)?;

    let mut wordmap: FxHashMap<String, f32> = <FxHashMap<String, f32>>::default();

    // Count records
    let mut count: i32 = 0;

    let stopwords: FxHashSet<String> = get_stop_words();

    // Iterating through records and deserializing into Tweet struct
    for result in reader.deserialize() {
        let record: TrainTweet = result?;

        // Tokenizing tweets and processing each word to remove punctuation
        let collect: FxHashSet<String> = record
            .tweet
            .split_word_bounds()
            .filter_map(|w| process_word(w, &get_special_char_regex(), &stopwords))
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

    println!("\nRead {} training tweets", count);

    Ok(wordmap)
}

// Reads in test dataset and builds map of tweet ids and predicted sentiment
fn tweet_predictor(
    path: &str,
    wordmap: &FxHashMap<String, f32>,
) -> Result<FxHashMap<u64, u8>, Box<dyn Error>> {
    // Read in test data
    let mut reader = csv::Reader::from_path(path)?;
    let _ = reader.headers();

    let mut tweet_predictions: FxHashMap<u64, u8> = <FxHashMap<u64, u8>>::default();

    // Count records
    let mut count: i32 = 0;

    for result in reader.deserialize() {
        let record: TestTweet = result?;

        let stopwords: FxHashSet<String> = get_stop_words();

        // Tokenizing tweets and processing each word to remove punctuation
        let collect: FxHashSet<String> = record
            .tweet
            .split_word_bounds()
            .filter_map(|w| process_word(w, &get_special_char_regex(), &stopwords))
            .collect();

        let mut tweet_score: f32 = 0.0;
        // Compare words in test data tweet with wordmap
        for word in collect {
            if wordmap.contains_key(&word) {
                // Get the word value
                // Add up scores of words in the tweets
                tweet_score += wordmap.get(&word).unwrap();
            }
        }
        // Push the sentiment guess and tweet ID into tweet_predictions map
        if tweet_score > 50.0 {
            tweet_predictions.entry(record.id).or_insert(4);
        }
        tweet_predictions.entry(record.id).or_insert(0);

        // Discarding unused struct values
        let _ = record.query;
        let _ = record.date;
        let _ = record.user;

        count += 1;
    }

    // Printing predictions map
    let mapcount = tweet_predictions.len();
    println!("\n\nLength of tweet_predictions map: {}", mapcount);

    println!("\nRead {} testing tweets", count);

    Ok(tweet_predictions)
}

// Read in test dataset to calculate accuracy
fn calculate_accuracy(
    path: &str,
    tweetpredictions: &FxHashMap<u64, u8>,
) -> Result<(), Box<dyn Error>> {
    // Reading from csv path
    let mut reader = csv::Reader::from_path(path)?;

    let mut test_sent_ids: FxHashMap<u64, u8> = <FxHashMap<u64, u8>>::default();

    let _ = reader.headers();

    let total_tweets = tweetpredictions.len();
    let mut correct_predictions = 0;

    for result in reader.deserialize() {
        let record: TestSentID = result?;

        let id = record.id;
        let sentiment = record.sentiment;

        test_sent_ids.entry(id).or_insert(sentiment);
    }

    println!("\n\nComparing predictions with real values & calculating accuracy...");
    for (tweet_id, prediction) in tweetpredictions {
        if let Some(real_sent) = test_sent_ids.get(tweet_id) {
            if prediction == real_sent {
                correct_predictions += 1;
            }
        }
    }

    let accuracy = (correct_predictions as f64) / (total_tweets as f64) * 100.0;

    println!("\nAccuracy: {:.2}%", accuracy);

    Ok(())
}

fn main() {
    // Creating hashmap for words and their score
    let start_time = Instant::now();
    let word_map = build_word_score_map("./sent_analysis_data/train_dataset_20k.csv");
    if let Err(e) = &word_map {
        eprintln!("{}", e);
    }

    let tweetpredictions = tweet_predictor(
        "./sent_analysis_data/test_dataset_10k.csv",
        &word_map.unwrap(),
    );
    if let Err(e) = &tweetpredictions {
        eprintln!("{}", e);
    }

    if let Err(e) = calculate_accuracy(
        "./sent_analysis_data/test_dataset_sentiment_10k.csv",
        &tweetpredictions.unwrap(),
    ) {
        eprintln!("{}", e);
    }

    let elapsed_time = start_time.elapsed();

    println!("\nTime to execute: {:?}", elapsed_time);
}
