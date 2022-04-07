use std::path::PathBuf;

fn read_file_or_die(file: PathBuf) -> String {
    return std::fs::read_to_string(&file)
        .unwrap_or_else(|e| panic!("could not read {:?}: {}", &file, e))
        .trim()
        .to_string()
}

pub fn read_cpu() -> String {
    return read_file_or_die(PathBuf::from_iter(["/", "proc", "loadavg"].iter()));
}

pub fn parse_cpu(s: &str) -> String {
    return s
        .split(" ")
        .nth(0)
        .expect("failed to split file contents")
        .to_string();
}

pub fn read_capacity(battery_dir: &PathBuf) -> String {
    return read_file_or_die(PathBuf::from_iter([battery_dir, &PathBuf::from("capacity")].iter()));
}

pub fn read_status(battery_dir: &PathBuf) -> String {
    return read_file_or_die(PathBuf::from_iter([battery_dir, &PathBuf::from("status")].iter()));
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_parse_cpu() {
        assert_eq!(
            parse_cpu("1.45 0.97 0.82 2/1297 160474"),
            "1.45"
        );
    }
}
