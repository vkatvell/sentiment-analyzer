use serde::Deserialize;
use std::error::Error;
use std::time::Instant;

#[derive(Debug, Deserialize)]
struct Tweet {
    sentiment: i8,
    id: i64,
    date: String,
    query: String,
    user: String,
    tweet: String,
}

fn read_from_file_serde(path: &str) -> Result<(), Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(path)?;

    let headers = reader.headers();
    println!("Headers: {:?}", headers);

    for result in reader.deserialize() {
        let _: Tweet = result?;
    }
    println!("read records");

    Ok(())
}

fn read_from_file(path: &str) -> Result<(), Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(path)?;

    let headers = reader.headers();
    println!("Headers: {:?}", headers);

    for result in reader.records() {
        let _ = result?;
    }

    println!("read records");

    Ok(())
}

fn main() {
    let serde_start_time = Instant::now();
    if let Err(e) = read_from_file_serde("./sent_analysis_data/train_dataset_20k.csv") {
        eprintln!("{}", e);
    }
    let serde_elapsed_time = serde_start_time.elapsed();

    println!("serde file read time: {:?}", serde_elapsed_time);

    let records_start_time = Instant::now();
    if let Err(e) = read_from_file("./sent_analysis_data/train_dataset_20k.csv") {
        eprintln!("{}", e);
    }
    let records_elapsed_time = records_start_time.elapsed();

    println!("regular file read time: {:?}", records_elapsed_time);

    // let headers: Vec<&str> = rows[0].split(',').collect();

    // let re = Regex::new(r#"([^",\r\n]+|"[^"\r\n]*")&"#).unwrap();

    // let mut data: Vec<Vec<&str>> = Vec::new();

    // for row in rows.iter().skip(1) {
    //     let fields: Vec<&str> = re
    //         .find_iter(row)
    //         .map(|m| m.as_str().trim_matches('"'))
    //         .collect();
    //     data.push(fields);
    // }

    // println!("Headers: {:?}", headers);

    // for row in data.iter() {
    //     println!("{:?}", row);
    // }
    // let test_string: SplitWhitespace =
    //     "This is a test to see what is tokenized from this string".split_whitespace();
    // let tokens: Vec<&str> = test_string.collect();

    // println!("{}", tokens.join("|"));
}
