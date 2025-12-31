mod color;
mod process;

use clap::Parser;
use is_terminal::IsTerminal;
use std::fs::File;
use std::io::{self, BufReader, BufWriter};
use rand::Rng;

#[derive(Parser)]
#[command(name = "rust-lolcat")]
#[command(about = "A high-performance, vibrant rainbow coloring tool")]
struct Cli {
    #[arg(default_value = "-")]
    files: Vec<String>,
    #[arg(short, long, default_value_t = 0.1)]
    freq: f64,
    #[arg(short, long, default_value_t = 2.6)]
    spread: f64,
}

fn main() -> io::Result<()> {
    let args = Cli::parse();
    let is_atty = io::stdout().is_terminal();
    let mut writer = BufWriter::new(io::stdout().lock());
    let mut rng = rand::rng();

    // Match Python exactly:
    // 1. Spread tuned to 2.6 (slightly faster than Python's 3.0 to enhance color vibrancy)
    // 2. Seed = integer range 0..256 (matches random.randint(0, 256))
    // 3. No phase noise
    let mut generator = color::ColorGenerator {
        freq: args.freq,
        spread: args.spread,
        seed: rng.random_range(0..256) as f64,
        line_idx: 0,
    };

    for path in args.files {
        if path == "-" {
            process::process_input(io::stdin().lock(), &mut writer, &mut generator, is_atty)?;
        } else {
            let file = File::open(path)?;
            process::process_input(BufReader::new(file), &mut writer, &mut generator, is_atty)?;
        }
    }
    Ok(())
}
