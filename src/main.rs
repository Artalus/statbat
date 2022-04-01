use std::path::PathBuf;
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
    return format!("{datetime},{capacity},{cpu},{status}\n")
}

fn read_csv_trimmed(filename: &PathBuf, max_lines: usize) -> String {
    return "".to_string();
}

fn write_csv(filename: &PathBuf, content: &str) {

}

fn main() {
    let args = args::Cli::parse();
    let csv_string = format_output(
        &get_date(),
        &read_capacity(&args.battery_dir),
        &parse_cpu(&read_cpu()),
        &read_status(&args.battery_dir)
    );
    if args.print {
        println!("{}", csv_string)
    }
    let mut csv = read_csv_trimmed(&args.log_path, args.log_max_lines);
    csv.push_str(&csv_string);
    write_csv(&args.log_path, &csv);
}
