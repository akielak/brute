use structopt::StructOpt;
use std::str::FromStr;

#[derive(Debug)]
pub enum Algorithm {
  MD5,
}

impl FromStr for Algorithm {
  type Err = String;
  fn from_str(string: &str) -> Result<Self, Self::Err> {
    match string {
      "md5" => Ok(Algorithm::MD5),
      _ => Err("Unexpected algorithm".to_string()),
    }
  }
}

#[derive(Debug, Clone)]
pub enum Alphabet {
  NUMERIC,
}

impl FromStr for Alphabet {
  type Err = String;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "numeric" => Ok(Alphabet::NUMERIC),
      _ => Err("Unexpected alphabet".to_string()),
    }
  }
}

#[derive(Debug, StructOpt)]
#[structopt(name = "MD5 brute")]
pub struct Args {
  #[structopt(long = "-min")]
  pub plaintext_min_length: usize,

  #[structopt(long = "-max")]
  pub plaintext_max_length: usize,

  #[structopt(
    long = "-algo",
    default_value = "md5",
    help = "Algorithms md5"
  )]
  pub algorithm: Algorithm,

  #[structopt(
    long = "-alpha",
    default_value = "numeric",
    help = "Alphabet numeric (0123456789)"
  )]
  pub alphabet: Alphabet,

  #[structopt(
  long = "-threads",
  default_value = "10",
  help = "Threads"
  )]
  pub threads: usize,

  #[structopt(
  long = "--hash",
  help = "Hash md5"
  )]
  pub hash: String,
}
