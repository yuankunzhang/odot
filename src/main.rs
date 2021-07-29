use clap::{AppSettings, Clap};

#[derive(Clap)]
#[clap(version = "1.0", author = "Yuankun Zhang <i@yuankun.me>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
  #[clap(short, long, default_value = "default.conf")]
  config: String,
  input: String,
  #[clap(short, long, parse(from_occurrences))]
  verbose: i32,
  #[clap(subcommand)]
  subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
  #[clap(version = "1.3", author = "Yuankun Zhang <i@yuankun.me>")]
  Test(Test),
}

#[derive(Clap)]
struct Test {
  #[clap(short)]
  debug: bool
}

fn main() {
    let opts: Opts = Opts::parse();
    println!("Value for config: {}", opts.config);
    println!("Using input file: {}", opts.input);

    match opts.verbose {
      0 => println!("No verbose info"),
      1 => println!("Some verbose info"),
      2 => println!("Tons of verbose info"),
      _ => println!("Don't be ridiculous"),
    }

    match opts.subcmd {
      SubCommand::Test(t) => {
        if t.debug {
          println!("Printing debug info...");
        } else {
          println!("Printing normally...");
        }
      }
    }
}
