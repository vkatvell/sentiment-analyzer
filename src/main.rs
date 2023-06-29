use serde::Deserialize;
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

fn read_from_file(path: &str) -> Result<(), Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(path)?;

    let headers = reader.headers();
    println!("Headers: {:?}", headers);

    let mut count: i32 = 0;

    for result in reader.deserialize() {
        let record: Tweet = result?;

        // Discarding values
        let _ = { record.query };
        let _ = { record.date };
        let _ = { record.user };

        println!(
            "Sentiment: {}, Tweet {}: {}",
            record.sentiment, record.id, record.tweet
        );

        count += 1;
    }

    println!("\nRead {} records", count);

    Ok(())
}

fn main() {
    let start_time = Instant::now();
    if let Err(e) = read_from_file("./sent_analysis_data/train_dataset_20k.csv") {
        eprintln!("{}", e);
    }
    let elapsed_time = start_time.elapsed();

    println!("\nTime: {:?}", elapsed_time);
}
