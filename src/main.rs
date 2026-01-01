mod color;
mod process;

use std::fs::File;
use std::io::{self, BufReader, BufWriter, IsTerminal};
use lexopt::prelude::*;

struct Args {
    files: Vec<String>,
    freq: f64,
    spread: f64,
}

fn parse_args() -> Result<Args, lexopt::Error> {
    let mut files = Vec::new();
    let mut freq = 0.1;
    let mut spread = 2.6;
    let mut parser = lexopt::Parser::from_env();

    while let Some(arg) = parser.next()? {
        match arg {
            Short('f') | Long("freq") => {
                freq = parser.value()?.parse()?;
            }
            Short('s') | Long("spread") => {
                spread = parser.value()?.parse()?;
            }
            Short('h') | Long("help") => {
                println!("lolcat-rs - A high-performance, vibrant rainbow coloring tool");
                println!("\nUsage: lolcat-rs [OPTIONS] [FILES]...");
                println!("\nOptions:");
                println!("  -f, --freq <FREQ>      Rainbow frequency [default: 0.1]");
                println!("  -s, --spread <SPREAD>  Rainbow spread [default: 2.6]");
                println!("  -h, --help             Print help");
                std::process::exit(0);
            }
            Value(val) => {
                files.push(val.into_string()?);
            }
            _ => return Err(arg.unexpected()),
        }
    }

    if files.is_empty() {
        files.push("-".to_string());
    }

    Ok(Args {
        files,
        freq,
        spread,
    })
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = parse_args()?;
    let is_atty = io::stdout().is_terminal();
    let mut writer = BufWriter::new(io::stdout().lock());

    // Enhanced seed: Mix nanoseconds with Process ID and apply a bit-mixer (SplitMix64 style)
    let seed = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| {
            let mut x = (d.as_nanos() as u64) ^ (std::process::id() as u64).rotate_left(32);
            // Simple mixing to spread entropy
            x = (x ^ (x >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
            x = (x ^ (x >> 27)).wrapping_mul(0x94D049BB133111EB);
            ((x ^ (x >> 31)) % 256) as f64
        })
        .unwrap_or(0.0);

    let mut generator = color::ColorGenerator {
        freq: args.freq,
        spread: args.spread,
        seed,
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
