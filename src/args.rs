use clap::Parser;

use std::path::PathBuf;

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
pub struct Cli {
    /// file to append data to
    #[clap(parse(from_os_str), short('f'), long, default_value="statbat.csv")]
    pub log_path: PathBuf,
    /// maximum lines sto store in file before truncating
    #[clap(short('m'), long, default_value_t=2*24*60/*minutes*/)]
    pub log_max_lines: usize,
    /// /sys/class file to read info from
    #[clap(parse(from_os_str), short, long, default_value="/sys/class/power_supply/BAT0")]
    pub battery_dir: std::path::PathBuf,
    /// read capacity percentage
    #[clap(short, long, parse(try_from_str = true_or_false), default_value="yes", value_names=&["yes/no"])]
    pub capacity: bool,
    /// read /proc/loadavg
    #[clap(short('u'), long, parse(try_from_str = true_or_false), default_value="yes", value_names=&["yes/no"])]
    pub cpu: bool,
    /// read status (discharging, charging, full)
    #[clap(short, long, parse(try_from_str = true_or_false), default_value="yes", value_names=&["yes/no"])]
    pub status: bool,
    /// read energy value
    #[clap(short, long, parse(try_from_str = true_or_false), default_value="no", value_names=&["yes/no"])]
    pub energy: bool,
    /// read power value
    #[clap(short, long, parse(try_from_str = true_or_false), default_value="no", value_names=&["yes/no"])]
    pub power: bool,
    /// print csv line to stdout
    #[clap(long)]
    pub print: bool,
}
