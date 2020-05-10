use clap::{crate_authors, crate_name, crate_version, App, AppSettings, Arg, SubCommand};
use std::fmt::{self, Display, Formatter};
use std::fs::File;
use std::io::{BufReader, Error};
use std::num::ParseIntError;
use std::path::Path;
use std::result::Result;

use grib::data::{Grib2, GribError};
use grib::reader::SeekableGrib2Reader;

enum CliError {
    GribError(GribError),
    ParseNumberError(ParseIntError),
    IOError(Error, String),
}

impl Display for CliError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::GribError(e) => write!(f, "{}", e),
            Self::ParseNumberError(e) => write!(f, "{:#?}", e),
            Self::IOError(e, path) => write!(f, "{}: {}", e, path),
        }
    }
}

impl From<GribError> for CliError {
    fn from(e: GribError) -> Self {
        Self::GribError(e)
    }
}

impl From<ParseIntError> for CliError {
    fn from(e: ParseIntError) -> Self {
        Self::ParseNumberError(e)
    }
}

fn app() -> App<'static, 'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::ColoredHelp)
        .subcommand(
            SubCommand::with_name("info")
                .about("Shows identification information")
                .arg(Arg::with_name("file").required(true)),
        )
        .subcommand(
            SubCommand::with_name("list")
                .about("Lists contained data")
                .arg(Arg::with_name("file").required(true)),
        )
        .subcommand(
            SubCommand::with_name("inspect")
                .about("Inspects and describes the data structure")
                .arg(
                    Arg::with_name("sections")
                        .help("Prints sections constructing the GRIB message")
                        .short("s")
                        .long("sections"),
                )
                .arg(
                    Arg::with_name("templates")
                        .help("Prints templates used in the GRIB message")
                        .short("t")
                        .long("templates"),
                )
                .arg(Arg::with_name("file").required(true))
                .after_help(
                    "\
This subcommand is mainly targeted at (possible) developers and
engineers, who wants to understand the data structure for the purpose
of debugging, enhancement, and education.\
",
                ),
        )
        .subcommand(
            SubCommand::with_name("decode")
                .about("Exports decoded data")
                .arg(Arg::with_name("file").required(true))
                .arg(Arg::with_name("index").required(true)),
        )
}

fn grib(file_name: &str) -> Result<Grib2<SeekableGrib2Reader<BufReader<File>>>, CliError> {
    let path = Path::new(file_name);
    let f = File::open(&path).map_err(|e| CliError::IOError(e, path.display().to_string()))?;
    let f = BufReader::new(f);
    Ok(Grib2::<SeekableGrib2Reader<BufReader<File>>>::read_with_seekable(f)?)
}

fn real_main() -> Result<(), CliError> {
    let matches = app().get_matches();

    match matches.subcommand() {
        ("info", Some(subcommand_matches)) => {
            let file_name = subcommand_matches.value_of("file").unwrap();
            let grib = grib(file_name)?;
            println!("{}", grib);
        }
        ("list", Some(subcommand_matches)) => {
            let file_name = subcommand_matches.value_of("file").unwrap();
            let grib = grib(file_name)?;
            println!("{:#?}", grib.submessages());
        }
        ("inspect", Some(subcommand_matches)) => {
            let file_name = subcommand_matches.value_of("file").unwrap();
            let grib = grib(file_name)?;

            let has_sect = subcommand_matches.is_present("sections");
            let has_tmpl = subcommand_matches.is_present("templates");

            let mut count = 0;
            if has_sect {
                count += 1;
            }
            if has_tmpl {
                count += 1;
            }
            let has_one = count == 1;
            let has_all = count == 0;

            let mut first = true;

            if has_sect || has_all {
                if !has_one {
                    println!("Sections:");
                }

                for sect in grib.sections().iter() {
                    println!("{}", sect);
                }
                first = false;
            }

            if has_tmpl || has_all {
                if !first {
                    println!("");
                }
                if !has_one {
                    println!("Templates:");
                }

                for tmpl in grib.list_templates() {
                    println!("{}", tmpl);
                }
            }
        }
        ("decode", Some(subcommand_matches)) => {
            let file_name = subcommand_matches.value_of("file").unwrap();
            let grib = grib(file_name)?;
            let index: usize = subcommand_matches.value_of("index").unwrap().parse()?;
            let values = grib.get_values(index)?;
            println!("{:#?}", values);
        }
        ("", None) => unreachable!(),
        _ => unreachable!(),
    }
    Ok(())
}

fn main() {
    if let Err(ref e) = real_main() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
