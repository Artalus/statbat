use clap::Parser;

use std::path::PathBuf;
use chrono::{Datelike, Timelike, Utc};
mod args;

fn read_file_or_die(file: PathBuf) -> String {
    return std::fs::read_to_string(&file)
        .unwrap_or_else(|e| panic!("could not read {:?}: {}", &file, e))
        .trim()
        .to_string()
}

fn read_cpu() -> String {
    return read_file_or_die(PathBuf::from_iter(["/", "proc", "loadavg"].iter()));
}

fn parse_cpu(s: &str) -> String {
    return s
        .split(" ")
        .nth(0)
        .expect("failed to split file contents")
        .to_string();
}

fn read_capacity(battery_dir: &PathBuf) -> String {
    return read_file_or_die(PathBuf::from_iter([battery_dir, &PathBuf::from("capacity")].iter()));
}

fn read_status(battery_dir: &PathBuf) -> String {
    return read_file_or_die(PathBuf::from_iter([battery_dir, &PathBuf::from("status")].iter()));
}

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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_cpu() {
        assert_eq!(
            parse_cpu("1.45 0.97 0.82 2/1297 160474"),
            "1.45"
        );
    }
}
