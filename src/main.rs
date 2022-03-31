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

fn read_cpu() -> String {
    let loadavg = PathBuf::from("/proc/loadavg");
    return std::fs::read_to_string(loadavg)
        .expect("could not read file")
        .split(" ")
        .nth(0)
        .expect("failed to split file contents")
        .to_string();
}

fn read_capacity(battery_dir: &PathBuf) -> String {
    let mut capacity = PathBuf::with_capacity(2);
    capacity.push(battery_dir);
    capacity.push("capacity");
    return std::fs::read_to_string(capacity)
        .expect("could not read file")
        .trim()
        .to_owned();
}

fn read_status(battery_dir: &PathBuf) -> String {
    let mut status = PathBuf::with_capacity(2);
    status.push(battery_dir);
    status.push("status");
    return std::fs::read_to_string(status)
        .expect("could not read file")
        .trim()
        .to_owned();
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
            &read_cpu(),
            &read_status(&args.battery_dir)
        )
    )
}
