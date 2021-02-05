use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "txs", about = "An example transaction processor")]
pub struct Options {
    /// Input csv file
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

pub fn get_options() -> Options {
    Options::from_args()
}
