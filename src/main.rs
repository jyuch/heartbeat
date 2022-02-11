use clap::Parser;
use heartbeat::http;
use std::process;

#[derive(Parser, Debug)]
#[clap(name = "heartbeat")]
#[clap(bin_name = "heartbeat")]
#[clap(version, about)]
struct Opts {
    #[clap(long)]
    url: String,
}

fn main() {
    let opts = Opts::parse();
    let result = http::get(opts.url.as_str());

    match result {
        Ok(true) => {
            process::exit(0);
        }
        Ok(false) => {
            eprint!("Unexpected status code.");
            process::exit(1);
        }
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    }
}
