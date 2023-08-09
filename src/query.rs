use std::fs::File;
use std::path::Path;
use std::time::SystemTime;
use clap::Parser;
use clap_derive::Parser;
use rayon::prelude::*;

use fst::{Error, IntoStreamer, Set, Streamer};
use memmap2::Mmap;

pub unsafe fn mmap_set<P: AsRef<Path>>(path: P) -> Result<Set<Mmap>, Error> {
    let mmap = Mmap::map(&File::open(path)?)?;
    let fst = Set::new(mmap)?;
    Ok(fst)
}


#[derive(Parser)]
struct Cli {
    #[arg(short = 'q', long = "queries")]
    queries: u64,
    #[arg(short = 'p', long = "prefix")]
    prefix: bool,
    #[arg(short = 'f', long = "from")]
    from: u64,
    #[arg(short = 't', long = "to")]
    to: u64,
    #[arg(short = 'i', long = "input")]
    path: String
}

fn main() {
    let args = Cli::parse();

    println!("Generating {}k random numbers' sha256 sums", args.queries / 1000);

    let sums: Vec<String> =
        (0..args.queries).into_par_iter()
            .map(|_| args.from + rand::random::<u64>() % args.to)
            .map(|v| v.to_string())
            // .map(sha256::digest)
            .map(|s| if args.prefix {
                //If we want prefix queries, just cut a random number of prefix characters
                (&s[0..(64 - rand::random::<usize>() % 64)]).to_string()
            } else {
                s
            })
            .collect();


    println!("Done");

    let set = unsafe { mmap_set(args.path).unwrap() };


    let start = SystemTime::now();
    println!("Querying {} values...", args.queries);

    let hits: Vec<usize> = sums.into_par_iter().map( |e| {
        let start = format!("{}{}", e, "0".repeat(64 - e.len()));
        let end = format!("{}{}", e, "f".repeat(64 - e.len()));

        let mut s = set.range().ge(start).le(end).into_stream();

        (0..5).into_iter().flat_map( |_| s.next().map(|v| String::from_utf8_lossy(v).to_string())).count()
    }
    ).collect();

    println!("Querying {} numbers finished in {} ms, resulting in {} qps", args.queries, start.elapsed().unwrap().as_millis(), args.queries / start.elapsed().unwrap().as_secs()  );

    let hits_string = (0..=5).map( |h| {
        let count = hits.iter().filter(|c| c == &&h).count();
        format!("{} hits with {} results", count, h)
    }).collect::<Vec<String>>().join(" ");

    println!("Query results: {}", hits_string);
}
