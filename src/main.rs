use std::error::Error;
use std::fs::File;

mod cli;
mod processor;

fn main() -> Result<(), Box<dyn Error>> {
    let options = cli::get_options();

    let file = File::open(options.input)?;
    let mut reader = csv::Reader::from_reader(file);

    let mut processor = processor::Processor::new();

    for message in reader.deserialize() {
        processor.process(message?);
    }

    let snapshot = processor.snapshot();
    dbg!(snapshot);

    Ok(())
}
