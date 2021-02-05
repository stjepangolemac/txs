use std::error::Error;
use std::fs::File;

mod cli;
mod processor;

fn main() -> Result<(), Box<dyn Error>> {
    let options = cli::get_options();

    let file = File::open(options.input)?;
    let reader = csv::Reader::from_reader(file);

    Ok(())
}
