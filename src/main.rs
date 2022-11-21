use clap::Parser;
use rspnt::telemetry::{get_subscriber, init_subscriber};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, author, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();
    let subscriber = get_subscriber("info");
    init_subscriber(subscriber);

    for _ in 0..args.count {
        tracing::info!("Hello {}!", args.name)
    }
}
