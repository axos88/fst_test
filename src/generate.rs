use std::fs::File;
use std::io::{BufWriter, Write};
use std::time::Instant;
use clap::Parser;
use clap_derive::Parser;
use pariter::{IteratorExt as _};
use sha2::Sha256;

use sha2::Digest;

#[derive(Parser)]
struct Cli {
    #[arg(short = 'f', long = "from")]
    from: u64,
    #[arg(short = 't', long = "to")]
    to: u64,
    #[arg(short = 'o', long = "output")]
    path: String
}

fn main() {
    let args = Cli::parse();

    let start = Instant::now();

    println!("Generating {}k sha256 sums", (args.to - args.from) / 1000);

    let f = File::create(args.path).unwrap();
    let mut w = BufWriter::with_capacity(1000 * 1000, f);

    (args.from..args.to).into_iter().parallel_map(|i| {
        let mut hasher = Sha256::default();
        hasher.update(i.to_string().as_bytes());
        let hash = hex::encode(hasher.finalize().as_slice());

        (i, hash)
    }).for_each(|(i,sha)| {
        if (i - args.from) % 100000 == 0 && i > args.from {
            println!("Generated {}k sha256 sums in {} seconds at {}k/s", (i - args.from) / 1000, start.elapsed().as_secs(), (i - args.from) / start.elapsed().as_millis().max(1) as u64);
        };

        w.write_all(format!("{}\n", sha).as_bytes()).unwrap();
    });

    w.flush().unwrap();

    println!("All done!");
}
