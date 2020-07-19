use reqwest::Client;
use serde::Deserialize;
use std::env;
use std::process;

// 0. Determine geolocation via ip?
// 1. Head request to get etag
// 2. Read etag, check file system if file exist
// 3. If it does, then read file, skip to step 5
// 4.a If it doesn't, delete old file
// 4.b download new file (use new etag as name)
// 5. display Rt value for given state?

#[derive(Debug, Deserialize)]
struct Record {
    date: String,
    region: String,
    index: String,
    mean: String,
    median: String,
    lower_80: String,
    upper_80: String,
    infections: String,
    test_adjusted_positive: String,
    test_adjusted_positive_raw: String,
    positive: String,
    tests: String,
    new_tests: String,
    new_cases: String,
    new_deaths: String,
}

pub struct Config {
    pub state: String,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();
        // TODO: validate given state
        let state = match args.next() {
            Some(arg) => arg,
            None => return Err("No state given!"),
        };

        Ok(Config { state })
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // send head request to determine if we should get the data
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    let url = "https://d14wlfuexuxgcm.cloudfront.net/covid/rt.csv";
    let client = Client::new();
    let csv_data = client.get(url).send().await?.text().await.unwrap();
    let csv_data = csv_data.as_bytes();
    let mut rdr = csv::Reader::from_reader(csv_data);
    let mut last: Option<Record> = None;
    for result in rdr.deserialize() {
        let record: Record = result?;
        if record.region == config.state {
            last = Some(record);
        }
    }
    if last.is_some() {
        println!("{:?}", last);
    }
    Ok(())
}
