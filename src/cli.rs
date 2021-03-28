use clap::{App, ArgMatches};
use lop::error::LopError::Cli;
use lop::error::{CliErrorType, LopError, LopResult};
use lop::range::RangeList;
use lop::{LopConfig, Mode};

pub fn build_cli<'a, 'b>() -> App<'a, 'b> {
    use clap::*;
    app_from_crate!()
        .arg(
            Arg::with_name("byte-list")
                .short("b")
                .takes_value(true)
                .allow_hyphen_values(true)
                .value_name("list")
                .help("list of byte positions to lop on"),
        )
        .arg(
            Arg::with_name("char-list")
                .short("c")
                .takes_value(true)
                .allow_hyphen_values(true)
                .value_name("list")
                .help("list of char positions to lop on"),
        )
        .arg(
            Arg::with_name("field-list")
                .short("f")
                .takes_value(true)
                .allow_hyphen_values(true)
                .value_name("list")
                .help("list of fields to lop on"),
        )
        .arg(
            Arg::with_name("delim")
                .short("d")
                .takes_value(true)
                .requires("field-list")
                .help("set the field delimiter (used with -f)"),
        )
        .arg(
            Arg::with_name("suppress")
                .short("s")
                .requires("field-list")
                .help("suppress lines with no delimiter characters (used with -f)"),
        )
        .arg(
            Arg::with_name("no-split")
                .short("n")
                .requires("byte-list")
                .help("do not split characters (used with -b)"),
        )
        .arg(Arg::with_name("file").multiple(true))
        .group(ArgGroup::with_name("list").required(true).args(&[
            "byte-list",
            "char-list",
            "field-list",
        ]))
}

pub fn parse_config(matches: &ArgMatches) -> LopResult<LopConfig> {
    let mode = parse_mode(matches)?;
    let ranges = matches
        .value_of("list")
        .ok_or(LopError::Cli(CliErrorType::MissingList))
        .and_then(|spec| RangeList::from_spec(spec))?;

    let paths = matches
        .values_of("file")
        .map(|values| values.map(String::from).collect());

    Ok(LopConfig::new(mode, ranges, paths))
}

fn parse_delim(delim: &Option<&str>) -> LopResult<String> {
    delim.map_or(Ok(String::from("\t")), |s| {
        if s.len() != 1 {
            Err(Cli(CliErrorType::BadDelimiter))
        } else {
            // we already checked the length is 1
            Ok(String::from(s))
        }
    })
}

#[derive(Debug)]
struct ModeSwitches {
    byte: bool,
    field: bool,
    char: bool,
}

fn parse_mode(matches: &ArgMatches) -> LopResult<Mode> {
    let mode_switches = ModeSwitches {
        byte: matches.is_present("byte-list"),
        char: matches.is_present("char-list"),
        field: matches.is_present("field-list"),
    };

    let mode = match mode_switches {
        ModeSwitches {
            byte: true,
            char: false,
            field: false,
        } => Mode::Byte(!matches.is_present("no-split")),
        ModeSwitches {
            char: true,
            byte: false,
            field: false,
        } => Mode::Char,
        ModeSwitches {
            field: true,
            char: false,
            byte: false,
        } => Mode::Field(
            parse_delim(&matches.value_of("delim"))?,
            matches.is_present("suppress"),
        ),
        _ => return Err(LopError::Cli(CliErrorType::UnknownMode)),
    };

    Ok(mode)
}

#[cfg(test)]
mod test {
    use super::*;
    use clap::AppSettings;

    #[test]
    fn test_parse_mode() {
        let mut cli = build_cli().global_setting(AppSettings::NoBinaryName);
        let any_list = "1-2";

        let args = &["-b", any_list];
        let matches = cli.get_matches_from_safe_borrow(args).unwrap();
        assert_eq!(Mode::Byte(true), parse_mode(&matches).unwrap());

        let args = &["-b", any_list, "-n"];
        let matches = cli.get_matches_from_safe_borrow(args).unwrap();
        assert_eq!(Mode::Byte(false), parse_mode(&matches).unwrap());

        let args = &["-c", any_list];
        let matches = cli.get_matches_from_safe_borrow(args).unwrap();
        assert_eq!(Mode::Char, parse_mode(&matches).unwrap());

        let args = &["-f", any_list];
        let matches = cli.get_matches_from_safe_borrow(args).unwrap();
        assert_eq!(
            Mode::Field(String::from("\t"), false),
            parse_mode(&matches).unwrap()
        );

        let args = &["-f", any_list, "-s"];
        let matches = cli.get_matches_from_safe_borrow(args).unwrap();
        assert_eq!(
            Mode::Field(String::from("\t"), true),
            parse_mode(&matches).unwrap()
        );

        let args = &["-f", any_list, "-d", ":"];
        let matches = cli.get_matches_from_safe_borrow(args).unwrap();
        assert_eq!(
            Mode::Field(String::from(":"), false),
            parse_mode(&matches).unwrap()
        );

        let args = &["-f", any_list, "-d", "::"];
        let matches = cli.get_matches_from_safe_borrow(args).unwrap();
        assert!(parse_mode(&matches).is_err());
    }
}
