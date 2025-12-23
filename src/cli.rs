use clap::Parser;
use std::path::PathBuf;

/// Scrape GameMaker data files from the GM48 Game Jam
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Directory the data files will be downloaded to
    #[arg(default_value = "gm48_datafiles")]
    pub directory: PathBuf,
}
