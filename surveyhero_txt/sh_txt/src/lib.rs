//! Converts SurveyHero markdown files to text files
//! with SurveyHero's text format
//!
//! NOTE: This is meant to be used to bootstrap
//! a survey; it is not meant to be a full solution
use std::{
    io::Read,
    path::{Path, PathBuf},
};

use color_eyre::eyre::{Result, eyre};

pub mod cli;
pub mod writer;

pub struct S2S<'a> {
    source: &'a Path,
    dist: PathBuf,
}

impl<'a> S2S<'a> {
    pub fn new(source: &'a Path, dist: PathBuf) -> Self {
        Self { source, dist }
    }

    pub fn run(&self) -> Result<()> {
        let mut file = std::fs::File::open(self.source)?;
        let mut contents = String::new();
        assert!(self.dist.is_dir());
        assert!(self.source.is_file());
        file.read_to_string(&mut contents)?;

        let questions = surveyhero::markdown::parse(&contents).map_err(|e| eyre!(e))?;
        let dist_file = self
            .dist
            .join(self.source.with_extension(".txt").file_name().unwrap());
        let mut dist_file = std::fs::File::create(dist_file)?;
        writer::write_questions(&questions, &mut dist_file)?;

        Ok(())
    }
}
