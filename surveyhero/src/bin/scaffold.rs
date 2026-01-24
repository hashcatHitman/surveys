use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;
use surveyhero::txt_writer::S2S;

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

fn main() -> Result<()> {
    let cli = Cli::parse();
    let dist = cli
        .dist()
        .cloned()
        .unwrap_or_else(|| cli.source().parent().unwrap().to_path_buf());
    let s2s = S2S::new(cli.source(), dist);
    s2s.run()
}
