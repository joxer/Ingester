use chrono::prelude::*;
use clap::{App, AppSettings, Arg, SubCommand};
use ingester::bitfinex::{self};
use ingester::exporter::{self};
use std::process::exit;

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .setting(AppSettings::DisableHelpSubcommand)
        .setting(AppSettings::VersionlessSubcommands)
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .value_name("FILE")
                .help("Sets a file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("start")
                .short("s")
                .long("start")
                .value_name("START")
                .help("start date")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("end")
                .short("e")
                .long("end")
                .value_name("END")
                .help("end date")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("resolution")
                .short("r")
                .long("resolution")
                .value_name("RESOLUTION")
                .help("resolution")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("symbol")
                .short("y")
                .long("symbol")
                .value_name("SYMBOL")
                .help("symbol")
                .takes_value(true),
        )
        .get_matches();
    let symbol = matches.value_of("symbol").unwrap();
    let file = matches.value_of("file").unwrap();
    let fstart = matches.value_of("start").unwrap();
    let fend = matches.value_of("end").unwrap();
    let resolution = matches.value_of("resolution").unwrap();
    println!("{}, {}, {}, {}", file, fstart, fend, resolution);
    let start = &fstart.parse::<DateTime<Utc>>().ok().unwrap_or_else(|| {
        println!("start value not parsed!");
        exit(2);
    });
    let end = &fend.parse::<DateTime<Utc>>().ok().unwrap_or_else(|| {
        println!("end value not parsed!");
        exit(2);
    });
    let data = bitfinex::Bitfinex::get(
        resolution.to_string(),
        symbol.to_string(),
        start.timestamp_millis(),
        end.timestamp_millis(),
    );
    let data_string = serde_json::to_string(&data).unwrap();
    let output_filename = file.to_string();
    exporter::JSONExporter::emit(output_filename, data_string);
    exit(0);
}
