use structopt::StructOpt;
use anyhow::Result;

#[derive(StructOpt, Debug)]
#[structopt(name="tdcat")]
struct CLIOptions {
  /// Increase verbosity of logging output.
  #[structopt(short="v", long="verbose", parse(from_occurrences))]
  verbosity: usize,
  /// Silence non-error logging
  #[structopt(short="q", long="quiet")]
  quiet: bool,
}

fn main() -> Result<()> {
  let opts = CLIOptions::from_args();
  stderrlog::new().quiet(opts.quiet).verbosity(opts.verbosity).init()?;
  Ok(())
}
