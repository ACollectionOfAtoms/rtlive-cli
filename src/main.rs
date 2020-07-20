use rtlive::run;
use rtlive::Config;
use std::env;
use std::process;

// 0. Determine geolocation via ip?
// 1. Head request to get etag
// 2. Read etag, check file system if file exist
// 3. If it does, then read file, skip to step 5
// 4.a If it doesn't, delete old file
// 4.b download new file (use new etag as name)
// 5. display Rt value for given state?

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    run(config).await?;
    Ok(())
}
