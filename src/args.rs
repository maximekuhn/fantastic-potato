use clap::Parser;

#[derive(Debug, Parser)]
pub struct Args {
    /// Configuration file path
    #[arg(long)]
    pub config_file_path: String,
}
