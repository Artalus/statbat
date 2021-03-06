use clap::Parser;

use chrono::{Datelike, Timelike, Local};

mod args;
mod fileworks;
use fileworks::*;
mod csv;

fn get_date() -> String {
    let now = Local::now().naive_local();
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
    let csv_string = format_output(
        &get_date(),
        &read_capacity(&args.battery_dir),
        &parse_cpu(&read_cpu()),
        &read_status(&args.battery_dir)
    );
    if args.print {
        println!("{}", csv_string)
    }
    let mut csv = csv::read_trimmed(&args.log_path, args.log_max_lines);
    csv.push(csv_string);
    csv::write(&args.log_path, csv);
}
