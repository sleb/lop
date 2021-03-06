use crate::error::LopResult;
use crate::range::RangeList;
use crate::Mode::Field;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

pub mod error;
pub mod range;

#[derive(Debug, Eq, PartialEq)]
pub enum Mode {
    Byte(bool),
    Field(String, bool),
    Char,
}

impl Mode {
    fn should_split_lines(&self) -> bool {
        match self {
            Field(delim, _) => delim != "\n",
            _ => true,
        }
    }
}

#[derive(Debug)]
pub struct LopConfig {
    range_set: RangeList,
    mode: Mode,
    paths: Option<Vec<String>>,
}

impl LopConfig {
    pub fn new(mode: Mode, range_set: RangeList, paths: Option<Vec<String>>) -> Self {
        LopConfig {
            mode,
            range_set,
            paths,
        }
    }
}

pub fn run(config: &LopConfig) -> LopResult<()> {
    let stdin = io::stdin();
    let stdin_path = "-";

    let stdout = io::stdout();
    let mut out = stdout.lock();

    let default_paths = vec![String::from(stdin_path)];
    let mut saw_stdin = false;

    let paths = config.paths.as_ref().unwrap_or(&default_paths);

    for path in paths {
        let mut input: Box<dyn BufRead> = if path == stdin_path {
            if saw_stdin {
                continue;
            }
            saw_stdin = true;
            Box::new(BufReader::new(stdin.lock()))
        } else {
            Box::new(BufReader::new(File::open(path)?))
        };

        if *&config.mode.should_split_lines() {
            for line in input.lines() {
                lop(&line?, config, &mut out)?;
                writeln!(&mut out, "")?;
            }
        } else {
            let mut line = String::new();
            input.read_to_string(&mut line)?;
            lop(&line, config, &mut out)?;
            writeln!(&mut out, "")?;
        }
    }

    Ok(())
}

fn lop<T: Write>(line: &str, config: &LopConfig, out: &mut T) -> LopResult<()> {
    match &config.mode {
        Mode::Byte(true) => lop_bytes(line, &config.range_set, out),
        Mode::Byte(false) => lop_bytes_on_char(line, &config.range_set, out),
        Mode::Char => lop_chars(line, &config.range_set, out),
        Mode::Field(delim, suppress) => lop_fields(line, delim, *suppress, &config.range_set, out),
    }
}

fn lop_bytes<T: Write>(line: &str, ranges: &RangeList, out: &mut T) -> LopResult<()> {
    let v: Vec<_> = line
        .bytes()
        .enumerate()
        .filter_map(|(i, b)| {
            let index = i + 1;
            if ranges.contains(index) {
                Some(b)
            } else {
                None
            }
        })
        .collect();

    out.write_all(&v)?;
    Ok(())
}

fn lop_bytes_on_char<T: Write>(line: &str, ranges: &RangeList, out: &mut T) -> LopResult<()> {
    let mut index = 1;
    let v: Vec<_> = line
        .chars()
        .filter_map(|c| {
            let high = index + c.len_utf8() - 1;
            index = high + 1;

            if ranges.contains(high) {
                Some(c)
            } else {
                None
            }
        })
        .flat_map(|c| {
            let mut buf = [0; 4];
            let s = c.encode_utf8(&mut buf);
            s.bytes().collect::<Vec<_>>()
        })
        .collect();

    out.write_all(&v)?;
    Ok(())
}

fn lop_chars<T: Write>(line: &str, ranges: &RangeList, out: &mut T) -> LopResult<()> {
    let v: Vec<_> = line
        .chars()
        .enumerate()
        .filter_map(|(i, c)| {
            let index = i + 1;
            if ranges.contains(index) {
                Some(c)
            } else {
                None
            }
        })
        .flat_map(|c| {
            let mut buf = [0; 4];
            let s = c.encode_utf8(&mut buf);
            s.bytes().collect::<Vec<_>>()
        })
        .collect();

    out.write_all(&v)?;
    Ok(())
}

fn lop_fields<T: Write>(
    line: &str,
    delim: &str,
    suppress: bool,
    ranges: &RangeList,
    out: &mut T,
) -> LopResult<()> {
    let mut contains_delim = false;
    let fields: Vec<_> = line
        .split(delim)
        .enumerate()
        .filter_map(|(i, f)| {
            contains_delim = i > 0;
            if ranges.contains(i + 1) {
                Some(f)
            } else {
                None
            }
        })
        .collect();

    let mut v: String = fields.join(delim);

    if !contains_delim {
        if suppress {
            v.clear();
        } else {
            v = String::from(line);
        }
    }

    out.write_all(v.as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::range::Range;

    #[test]
    fn lop_bytes_ascii() {
        let s = String::from("123456");
        let mut buf = Vec::new();
        let ranges = RangeList::from(vec![Range::Singleton(3)]);
        lop_bytes(&s, &ranges, &mut buf).unwrap();

        assert_eq!(vec![b'3'], buf);

        let s = String::from("123456");
        let mut buf = Vec::new();
        let ranges = RangeList::from(vec![Range::Inclusive(3, 5)]);
        lop_bytes(&s, &ranges, &mut buf).unwrap();

        assert_eq!(vec![b'3', b'4', b'5'], buf);

        let s = String::from("123456");
        let mut buf = Vec::new();
        let ranges = RangeList::from(vec![Range::Singleton(2), Range::From(5)]);
        lop_bytes(&s, &ranges, &mut buf).unwrap();

        assert_eq!(vec![b'2', b'5', b'6'], buf);
    }

    #[test]
    fn lop_bytes_multi_byte() {
        let s = String::from("?????????????????????");
        let mut buf = Vec::new();
        let ranges = RangeList::from(vec![Range::To(2)]);
        lop_bytes(&s, &ranges, &mut buf).unwrap();

        assert_eq!(vec![0xE3_u8, 0x81_u8], buf);
    }

    #[test]
    fn lop_chars_ascii() {
        let s = String::from("123456");
        let mut buf = Vec::new();
        let ranges = RangeList::from(vec![Range::Singleton(3)]);
        lop_bytes(&s, &ranges, &mut buf).unwrap();

        assert_eq!(vec![b'3'], buf);

        let s = String::from("123456");
        let mut buf = Vec::new();
        let ranges = RangeList::from(vec![Range::Inclusive(3, 5)]);
        lop_bytes(&s, &ranges, &mut buf).unwrap();

        assert_eq!(vec![b'3', b'4', b'5'], buf);

        let s = String::from("123456");
        let mut buf = Vec::new();
        let ranges = RangeList::from(vec![Range::Singleton(2), Range::From(5)]);
        lop_chars(&s, &ranges, &mut buf).unwrap();

        assert_eq!(vec![b'2', b'5', b'6'], buf);
    }

    #[test]
    fn lop_chars_multi_byte() {
        let s = String::from("??????????????????");
        for (i, c) in s.chars().enumerate() {
            let index = i + 1;
            let mut buf = Vec::new();
            let ranges = RangeList::from(vec![Range::Singleton(index)]);
            lop_chars(&s, &ranges, &mut buf).unwrap();

            assert_eq!(format!("{}", c).bytes().collect::<Vec<_>>(), buf);
        }
    }

    #[test]
    fn lop_bytes_on_char_ascii() {
        let s = String::from("123456");

        let mut buf = Vec::new();
        let ranges = RangeList::from(vec![Range::Singleton(3)]);
        lop_bytes_on_char(&s, &ranges, &mut buf).unwrap();

        assert_eq!(vec![b'3'], buf);

        let mut buf = Vec::new();
        let ranges = RangeList::from(vec![Range::Inclusive(3, 5)]);
        lop_bytes_on_char(&s, &ranges, &mut buf).unwrap();

        assert_eq!(vec![b'3', b'4', b'5'], buf);

        let mut buf = Vec::new();
        let ranges = RangeList::from(vec![Range::Singleton(2), Range::From(5)]);
        lop_bytes_on_char(&s, &ranges, &mut buf).unwrap();

        assert_eq!(vec![b'2', b'5', b'6'], buf);
    }

    #[test]
    fn lop_bytes_on_char_multi_byte() {
        let s = String::from("?????????????????????");

        let mut buf = Vec::new();
        let ranges = RangeList::from(vec![Range::Singleton(3)]);
        lop_bytes_on_char(&s, &ranges, &mut buf).unwrap();

        assert_eq!(s.as_bytes()[0..3], buf);

        let mut buf = Vec::new();
        let ranges = RangeList::from(vec![Range::Inclusive(3, 5)]);
        lop_bytes_on_char(&s, &ranges, &mut buf).unwrap();

        assert_eq!(s.as_bytes()[0..3], buf);

        let mut buf = Vec::new();
        let ranges = RangeList::from(vec![Range::Inclusive(3, 6)]);
        lop_bytes_on_char(&s, &ranges, &mut buf).unwrap();

        assert_eq!(s.as_bytes()[0..6], buf);
    }

    #[test]
    fn lop_fields_default_delimiter() {
        let s = String::from("a\tb\tc");

        let mut buf = Vec::new();
        let ranges = RangeList::from(vec![Range::From(2)]);
        lop_fields(&s, "\t", false, &ranges, &mut buf).unwrap();

        assert_eq!(s.as_bytes()[2..], buf);
    }
}
