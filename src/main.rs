use anyhow::Result;
use std::fs::File;
use std::io::stdout;

mod cli;
mod processor;

fn main() -> Result<()> {
    let options = cli::get_options();

    let file = File::open(options.input_file)?;
    let mut reader = csv::Reader::from_reader(file);

    let mut processor = processor::Processor::new();

    reader
        .deserialize()
        .for_each(|message| processor.process(message.expect("Could not read row in csv")));

    let snapshot = processor.snapshot();
    let mut wtr = csv::Writer::from_writer(stdout());

    wtr.write_record(&["client", "available", "held", "total", "locked"])?;

    snapshot.iter().for_each(|(client_id, account)| {
        wtr.serialize((
            client_id,
            account.available,
            account.held,
            account.total(),
            account.frozen,
        ))
        .expect("Account could not be written to output");
    });

    wtr.flush()?;

    Ok(())
}
