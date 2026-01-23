use clap::Parser;
use color_eyre::Result;
use sh_txt::S2S;
use sh_txt::cli::Cli;

fn main() -> Result<()> {
    let cli = Cli::parse();
    let dist = cli
        .dist()
        .cloned()
        .unwrap_or_else(|| cli.source().parent().unwrap().to_path_buf());
    let s2s = S2S::new(cli.source(), dist);
    s2s.run()
}
