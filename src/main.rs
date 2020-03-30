use clap::{crate_authors, crate_name, crate_version, App, Arg, SubCommand};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::result::Result;

use rust_grib2::parser::{Grib2FileReader, GribReader, ParseError};

enum CliError {
    NoSubCommandSpecified,
    ParseError(ParseError),
}

impl Display for CliError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::NoSubCommandSpecified => write!(f, "No subcommand specified"),
            Self::ParseError(e) => write!(f, "{:#?}", e),
        }
    }
}

impl From<ParseError> for CliError {
    fn from(e: ParseError) -> Self {
        Self::ParseError(e)
    }
}

fn app() -> App<'static, 'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
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
}

fn grib(file_name: &str) -> Result<Grib2FileReader<BufReader<File>>, ParseError> {
    let path = Path::new(file_name);
    let display = path.display();

    let f = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why.to_string()),
        Ok(f) => f,
    };
    let f = BufReader::new(f);

    Grib2FileReader::new(f)
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
            println!("{:#?}", grib.list_submessages().unwrap());
        }
        ("", None) => return Err(CliError::NoSubCommandSpecified),
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
