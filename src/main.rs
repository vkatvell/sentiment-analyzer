use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::time::Instant;
#[derive(Debug, Deserialize)]
struct Tweet {
    sentiment: u8,
    id: u64,
    date: String,
    query: String,
    user: String,
    tweet: String,
}

fn read_from_file(path: &str, mut wordmap: HashMap<String, f32>) -> Result<(), Box<dyn Error>> {
    // Reading from csv path
    let mut reader = csv::Reader::from_path(path)?;

    // Count records
    let mut count: i32 = 0;

    // Iterating through records and deserializing into Tweet struct
    for result in reader.deserialize() {
        let record: Tweet = result?;

        // Tokenizing tweets
        let collection: Vec<&str> = record.tweet.split_whitespace().collect();

        // Scoring words higher if pos. sentiment
        for word in &collection {
            if record.sentiment == 4 {
                *wordmap.entry(word.to_string()).or_insert(0.0) += 5.0;
            }
            *wordmap.entry(word.to_string()).or_insert(0.0) -= 1.0;
        }

        // Discarding unused struct values
        let _ = record.query;
        let _ = record.date;
        let _ = record.user;
        let _ = record.id;
        let _ = record.sentiment;

        count += 1;
    }

    println!("\n\nWords and their score: {:?}", wordmap);

    println!("\nRead {} records", count);

    Ok(())
}

fn main() {
    // Creating hashmap for words and their score
    let word_map: HashMap<String, f32> = HashMap::new();

    let start_time = Instant::now();
    if let Err(e) = read_from_file("./sent_analysis_data/train_dataset_10val.csv", word_map) {
        eprintln!("{}", e);
    }
    let elapsed_time = start_time.elapsed();

    println!("\nTime: {:?}", elapsed_time);
}
