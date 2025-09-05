use clap::Parser;

#[derive(Parser)]
#[command(version, about)]
pub struct Config {
    #[arg(long, default_value_t = 3000)]
    pub port: u16,
    #[arg(long, default_value = "0.0.0.0")]
    pub host: String,
    #[arg(long)]
    pub file: String,
}

impl Config {
    pub fn new() -> Config {
        Config::parse()
    }
}
