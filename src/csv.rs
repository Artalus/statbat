use std::fs::*;
use std::io::{prelude::*, BufReader};
use std::path::Path;

fn prepend(v: &mut Vec<String>, x: &str) {
    let slice = &[x.to_string()];
    v.splice(0..0, slice.iter().cloned());
}

fn trim_csv(lines: &mut Vec<String>, max_lines: usize) {
    let length = lines.len();
    if length <= max_lines {
        return;
    }
    let header = lines
        .drain(0..=0)
        .next()
        .expect("no lines in csv");
    if !header.starts_with("# ") {
        panic!("csv should start with a #header!")
    }

    let to_drop = length - max_lines;
    lines.drain(0..to_drop);
    prepend(lines, &header);
}

pub fn read_trimmed<P>(filename: &P, max_lines: usize) -> Vec<String>
where
    P: AsRef<Path> // to pass both "literals" and path_vars
{
    if !filename.as_ref().exists() {
        return vec!["# auto generated header".to_owned()];
    }
    let file = File::open(filename)
        .unwrap_or_else(|e| panic!("Failed to open {:?}: {e}", filename.as_ref()));
    let mut lines: Vec<String> = BufReader::new(file)
        .lines()
        .map(
            |l| l.unwrap_or_else(|e| panic!("Could not read lines: {e}"))
        )
        .collect();
    trim_csv(&mut lines, max_lines);
    return lines;
}

pub fn write<P>(filename: &P, lines: Vec<String>)
where
    P: AsRef<Path> // to pass both "literals" and path_vars
{
    let file = File::create(filename)
        .unwrap_or_else(|e| panic!("could not create file {:?}: {e}", filename.as_ref()));
    for ln in &lines {
        let wrt = writeln!(&file, "{}", &ln);
        match wrt {
            Ok(_) => {}
            Err(e) => {
                println!("Failed to write to file: {}", &e);
                break;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn prepend_empty() {
        let mut lines = vec![];
        prepend(&mut lines, "x");
        assert_eq!(lines, vec!["x"]);
    }

    #[test]
    fn prepend_1() {
        let mut lines = vec!["1".to_string()];
        prepend(&mut lines, "2");
        assert_eq!(lines, vec!["2", "1"]);
    }
    #[test]

    fn prepend_2() {
        let mut lines = vec![
            "1".to_string(),
            "2".to_string(),
        ];
        prepend(&mut lines, "3");
        assert_eq!(lines, vec!["3", "1", "2"]);
    }

    #[test]
    fn trim_for_1() {
        let mut lines = vec![
            "# header".to_string(),
            "1,".into(),
            "2,".into(),
            "3,".into(),
        ];
        trim_csv(&mut lines, 1);
        assert_eq!(
            lines,
            vec![
                "# header",
            ]
        );
    }

    #[test]
    fn trim_for_2() {
        let mut lines = vec![
            "# header".to_string(),
            "1,".into(),
            "2,".into(),
            "3,".into(),
        ];
        trim_csv(&mut lines, 2);
        assert_eq!(
            lines,
            vec![
                "# header",
                "3,",
            ]
        );
    }

    #[test]
    fn trim_for_3() {
        let mut lines = vec![
            "# header".to_string(),
            "1,".into(),
            "2,".into(),
            "3,".into(),
        ];
        trim_csv(&mut lines, 3);
        assert_eq!(
            lines,
            vec![
                "# header",
                "2,",
                "3,",
            ]
        );
    }

    #[test]
    fn trim_for_4() {
        let mut lines = vec![
            "# header".to_string(),
            "1,".into(),
            "2,".into(),
            "3,".into(),
        ];
        trim_csv(&mut lines, 4);
        assert_eq!(
            lines,
            vec![
                "# header",
                "1,",
                "2,",
                "3,"
            ]
        );
    }

    #[test]
    fn trim_for_5() {
        let mut lines = vec![
            "# header".to_string(),
            "1,".into(),
            "2,".into(),
            "3,".into(),
        ];
        trim_csv(&mut lines, 5);
        assert_eq!(
            lines,
            vec![
                "# header",
                "1,",
                "2,",
                "3,",
            ]
        );
    }
}
