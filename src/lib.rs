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

async fn fetch_data() -> Result<String, reqwest::Error> {
  // send head request to determine if we should get the data
  let url = "https://d14wlfuexuxgcm.cloudfront.net/covid/rt.csv";
  let client = Client::new();
  client.get(url).send().await?.text().await
}

fn get_latest_record_for_state(csv_data: String, state: String) -> Option<Record> {
  let csv_data = csv_data.as_bytes();
  let mut rdr = csv::Reader::from_reader(csv_data);
  let mut last: Option<Record> = None;
  for result in rdr.deserialize() {
    let record: Record = match result {
      Ok(v) => v,
      Err(_) => panic!("could not read csv data!"),
    };
    if record.region == state {
      last = Some(record);
    }
  }
  last
}

pub async fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let csv_data = fetch_data().await?;
  let last = get_latest_record_for_state(csv_data, config.state);
  if last.is_some() {
    println!("{}", pretty_summary(last.unwrap()));
  };
  Ok(())
}

fn pretty_summary(d: Record) -> String {
  // TODO: Error handling
  let parse_or_unknown = |s: String| match s.parse::<f64>() {
    Ok(v) => format!("{0:.2}", v),
    Err(_) => String::from("Unknown"),
  };
  format!(
    "Region: {region}
Rt: {rt}
Infection Count: {infection_count}
New Cases: {new_cases}
New Deaths: {new_deaths}
Date: {date}",
    region = d.region,
    rt = parse_or_unknown(d.mean),
    infection_count = parse_or_unknown(d.infections),
    new_cases = parse_or_unknown(d.new_cases),
    new_deaths = parse_or_unknown(d.new_deaths),
    date = d.date
  )
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn gets_latest_record_for_state() {
    let contents = String::from("\
date,region,index,mean,median,lower_80,upper_80,infections,test_adjusted_positive,test_adjusted_positive_raw,positive,tests,new_tests,new_cases,new_deaths
2020-03-02,ME,0,1.372039701105285,1.3567493753249258,1.1676226928373998,1.5628004513732852,52.729115945543,0.0,0.0,0.0,0.0,,,");
    let d = get_latest_record_for_state(contents, String::from("ME")).unwrap();
    assert_eq!(d.region, "ME");
  }

  #[test]
  fn pretty_summary_is_pretty() {
    let contents = String::from("\
date,region,index,mean,median,lower_80,upper_80,infections,test_adjusted_positive,test_adjusted_positive_raw,positive,tests,new_tests,new_cases,new_deaths
2020-03-02,ME,0,1.372039701105285,1.3567493753249258,1.1676226928373998,1.5628004513732852,52.729115945543,0.0,0.0,0.0,0.0,,,");
    let d = get_latest_record_for_state(contents, String::from("ME")).unwrap();
    let pretty = pretty_summary(d);
    assert_eq!(
      pretty,
      "Region: ME
Rt (mean): 1.37
Infection Count: 52.73
New Cases: Unknown
New Deaths: Unknown
Date: 2020-03-02"
    )
  }
}
