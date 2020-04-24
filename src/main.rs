mod args;
use args::Args;
use structopt::StructOpt;

mod find;
use find::find;

fn main() {
  let args: Args = Args::from_args();
  print!(
    "Search hash {} {:?} {:?} ",
    args.hash, args.algorithm, args.alphabet
  );
  print!(
    "Plaintext length {:?}-{:?} ",
    args.plaintext_min_length, args.plaintext_max_length
  );
  println!("Threads {:?}.", args.threads);
  find(
    args.plaintext_min_length,
    args.plaintext_max_length,
    args.algorithm,
    args.alphabet,
    args.threads,
    args.hash,
  );
}
