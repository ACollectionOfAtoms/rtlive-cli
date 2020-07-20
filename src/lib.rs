use reqwest::Client;
use serde::Deserialize;
use std::error::Error;

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

pub async fn run(config: Config) -> Result<(), Box<dyn Error>> {
  // send head request to determine if we should get the data
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
  };
  Ok(())
}
