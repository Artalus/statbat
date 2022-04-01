use clap::Parser;

use chrono::{Datelike, Timelike, Utc};

mod args;
mod fileworks;
use fileworks::*;

fn get_date() -> String {
    let now = Utc::now();
    return format!(
        "{}.{:02}.{:02} {:02}:{:02}:{:02}",
        now.year(),
        now.month(),
        now.day(),
        now.hour(),
        now.minute(),
        now.second(),
    );
}

fn format_output(
    datetime: &str,
    capacity: &str,
    cpu: &str,
    status: &str
) -> String {
    return format!("{datetime},{capacity},{cpu},{status}")
}


fn main() {
    let args = args::Cli::parse();
    println!("{}",
        format_output(
            &get_date(),
            &read_capacity(&args.battery_dir),
            &parse_cpu(&read_cpu()),
            &read_status(&args.battery_dir)
        )
    )
}
