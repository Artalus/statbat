use clap::Parser;

use std::path::PathBuf;
use chrono::{Datelike, Timelike, Utc};

fn true_or_false(s: &str) -> Result<bool, &'static str> {
    match s {
        "yes" => Ok(true),
        "y" => Ok(true),
        "true" => Ok(true),
        "1" => Ok(true),
        "no" => Ok(false),
        "n" => Ok(false),
        "false" => Ok(false),
        "0" => Ok(false),
        _ => Err("expected `true` or `false`"),
    }
}

/// Read current battery info and append it to .csv for future graphing
#[derive(Parser, Debug)]
struct Cli {
    /// file to append data to
    #[clap(parse(from_os_str), short('f'), long, default_value="statbat.csv")]
    log_path: PathBuf,
    /// maximum lines sto store in file before truncating
    #[clap(short('m'), long, default_value_t=24*60*2)]
    log_max_lines: u32,
    /// /sys/class file to read info from
    #[clap(parse(from_os_str), short, long, default_value="/sys/class/power_supply/BAT0")]
    battery_dir: std::path::PathBuf,
    /// read capacity percentage
    #[clap(short, long, parse(try_from_str = true_or_false), default_value="yes", value_names=&["yes/no"])]
    capacity: bool,
    /// read /proc/loadavg
    #[clap(short('u'), long, parse(try_from_str = true_or_false), default_value="yes", value_names=&["yes/no"])]
    cpu: bool,
    /// read status (discharging, charging, full)
    #[clap(short, long, parse(try_from_str = true_or_false), default_value="yes", value_names=&["yes/no"])]
    status: bool,
    /// read energy value
    #[clap(short, long, parse(try_from_str = true_or_false), default_value="no", value_names=&["yes/no"])]
    energy: bool,
    /// read power value
    #[clap(short, long, parse(try_from_str = true_or_false), default_value="no", value_names=&["yes/no"])]
    power: bool,
}

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
    let args = Cli::parse();
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
