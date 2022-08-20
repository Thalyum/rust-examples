mod cli;
mod prio;

use verbosity::Verbosity;

fn main() {
    let matches = cli::build_cli().get_matches();

    let verbose = if matches.contains_id("verbose") {
        Verbosity::Verbose
    } else {
        Verbosity::Quite
    };
    verbose.set_as_global();

    if matches.contains_id("extract") {
        if matches.contains_id("output") {
            eprintln!("'-o/--output' switch has no impact in extract mode");
        }
        prio::main_extract();
    } else {
        let output_path = matches.get_one::<String>("output").unwrap();
        let inputs = matches
            .get_many::<String>("inputs")
            .map(|vals| vals.collect::<Vec<_>>())
            .unwrap();
        prio::main_concat(inputs, output_path);
    }
}
