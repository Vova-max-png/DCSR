use std::path::PathBuf;
use clap::Parser;

/// Program to read docs
#[derive(Parser, Debug)]
#[command(author="Cat", version="0.0.1", about, long_about = None)]
pub struct Args {
    /// Paths to files
    pub url: String,
}