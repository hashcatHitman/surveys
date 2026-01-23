use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    #[clap(short, long)]
    pub source: PathBuf,

    #[clap(short, long)]
    pub dist: Option<PathBuf>,
}

impl Cli {
    pub fn source(&self) -> &PathBuf {
        &self.source
    }

    pub fn dist(&self) -> Option<&PathBuf> {
        self.dist.as_ref()
    }
}
