mod fuzz;

// ArgMatches, SubCommand
use std::result::Result;
use futures::executor::block_on;
use clap::{crate_authors, App, AppSettings, Arg};

fn main() {
  let mut app = App::new("TiKV Client (tikv-cli)")
    .about("TiKV Client is a TiKV's easy usage client.")
    .author(crate_authors!())
    .setting(AppSettings::AllowExternalSubcommands)
    .arg(
      Arg::with_name("pd")
        .long("pd")
        .takes_value(true)
        .help("Set the address of pd")
    )
    .arg(
      Arg::with_name("fuzz")
        .long("fuzz")
        .takes_value(true)
        .help("Do fuzz test")
    );

  let matches = app.clone().get_matches();
  if matches.args.is_empty() {
    let _ = app.print_help();
    println!("");
    return;
  }


  let mut pds = vec!{};
  if let Some(matches) = matches.value_of("pd") {
    pds = matches.split(",").collect::<Vec<&str>>()
  }

  if pds.len() == 0 {
    let _ = app.print_help();
    println!("");
  }

  if let Some(matches) = matches.value_of("fuzz") {
    if let Result::Ok(count) = matches.parse::<u32>() {
      let f = block_on(fuzz::Fuzz::new(pds));
      block_on(f.read_fuzz(count));
    } else {
      println!("fuzz should be a uint, got \"{}\"", matches)
    }
    return;
  }

  println!("No workload.");
}
