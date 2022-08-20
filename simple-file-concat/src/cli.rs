use clap::{App, Arg, ValueHint};

pub fn build_cli() -> App<'static> {
    App::new("simple-file-concat")
        .about("Simple file concatenator")
        .arg(
            Arg::new("extract")
                .short('e')
                .long("extract")
                .help("Extract a concatenated archive instead"),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .required_unless_present("extract")
                .takes_value(true)
                .value_hint(ValueHint::FilePath)
                .value_name("FILE")
                .help("output file"),
        )
        .arg(
            Arg::new("inputs")
                .required(true)
                .takes_value(true)
                .multiple_values(true)
                .value_hint(ValueHint::FilePath)
                .help("list of input files to process"),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("explain what is being done"),
        )
}
