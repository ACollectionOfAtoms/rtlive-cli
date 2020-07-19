# Rtlive CLI Stats

This is a utility to provide COVID-19 statistics from https://rt.live/ right in your terminal (so that you may experience existential dread even while you work!)

## Install

```sh
cargo install
```

## Run

```sh
cargo run <STATE>
```

State in this case is an abbreviation such as TX or NY.

### TODOS

- Clean up bad bad dirty code: Add lib, remove code from main.rs, add test
- Build project for distribution to osx
- Write function to show latest data (by parsing dates in csv; currently relies on csv ordering)
- Save the csv to disk and compare etag from headers to avoid extraneous request
- integrate https://github.com/fdehau/tui-rs and use the csv data to generate neat graphs
