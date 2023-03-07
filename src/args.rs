use std::path::PathBuf;
use clap::Parser;

/// Program to read docs
#[derive(Parser, Debug)]
#[command(author="Cat", version, about, long_about = None)]
pub struct Args {
    /// Paths to files
    #[arg(default_value_t = String::from("https://googlt.com"))]
    pub url: String,
    #[arg(default_value_t = String::from("dark"))]
    pub theme: String,
}