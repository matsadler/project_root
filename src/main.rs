use log::debug;
use structopt::StructOpt;

use project_root::{find_root, prepare_output, Exit};

/// Finds the root directory of a project from the current working directory,
/// based upon a marker file
#[derive(StructOpt, Debug)]
struct Opts {
    /// Verbose mode, multiples increase the verbosity
    #[structopt(short, long, parse(from_occurrences))]
    verbose: usize,
    /// Search directories top down rather than bottom up
    #[structopt(short, long)]
    top_down: bool,
    /// Return only the basename of the root directory
    #[structopt(short, long)]
    basename: bool,
    /// Marker file that appears in the root directory
    #[structopt(name = "FILE")]
    marker: String,
}

fn main() {
    let opts = Opts::from_args();

    stderrlog::new()
        .module(module_path!())
        .show_level(false)
        .color(stderrlog::ColorChoice::Never)
        .verbosity(opts.verbose)
        .init()
        .unwrap();

    debug!("{:#?}", opts);

    let root = find_root(&opts.marker, opts.top_down).exit(1);
    println!("{}", prepare_output(&root, opts.basename).exit(1));
}
