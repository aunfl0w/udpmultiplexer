use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
/// Multiplex incoming udp packets on listen host:port to
/// N target host:port locations
pub struct Args {
    /// Listen host:port use if ip address or 0.0.0.0 for all
    #[arg(long, required = true)]
    pub listen: String,

    /// Target host:port can repeat
    #[arg(long, required = true)]
    pub target: Vec<String>,

    /// Log count of incoming messages every log-count messages
    #[arg(short, long, required = false, default_value = "0")]
    pub log_count: u128,

    /// Log count of packet backlog
    #[arg(long, required = false, default_value = "false")]
    pub log_backlog: bool,

    /// Add statsd count of packets sent
    #[arg(short, long, required = false, default_value = "")]
    pub statsd_count_name: String,

    /// Duration to wait to flush threads in ms.  Can be used to tune throughput.
    #[arg(short, long, required = false, default_value = "10")]
    pub flush_time_ms: u64,
}

pub fn parse_args() -> Args {
    let args = Args::parse();

    println!("Listening on {}", args.listen);

    for to in &args.target {
        println!("Sending to {}", to);
    }

    if args.log_count > 0 {
        println!("Logging count every {} messages", args.log_count);
    }

    args
}
