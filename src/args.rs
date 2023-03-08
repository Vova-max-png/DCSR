use std::path::PathBuf;
use clap::Parser;

/// Program to read docs
#[derive(Parser, Debug)]
#[command(author="Cat", version, about, long_about = None)]
pub struct Args {
    /// Url to page
    #[arg(default_value_t = String::from("https://google.com"))]
    pub url: String,
}