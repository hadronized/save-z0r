use reqwest::blocking::{Client, Response};
use std::fs::File;
use std::io::Write as _;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct CLIOpt {
  #[structopt(
    long,
    default_value = "0",
    help = "The index of the first loop to scrap."
  )]
  from: usize,
  #[structopt(long, help = "The index of the last loop to scrap.")]
  to: usize,
  #[structopt(
    short = "d",
    long,
    default_value = "/tmp",
    help = "Where to put downloaded loops."
  )]
  dest_dir: String,
}

fn main() {
  //https://z0r.de/L/z0r-de_1.swf

  let cli_opt = CLIOpt::from_args();
  let client = Client::new();

  for i in cli_opt.from..cli_opt.to {
    let url = format!("https://z0r.de/L/z0r-de_{}.swf", i);

    println!("scrapping loop {} ({})…", i, url);
    match client.get(&url).send().and_then(Response::bytes) {
      Ok(swf_bytes) => {
        println!("\t…fetched");
        let dest_path = format!("{}/loop_{}.swf", cli_opt.dest_dir, i);

        match File::create(&dest_path) {
          Ok(mut file) => {
            let write = file.write_all(&swf_bytes);
            if write.is_err() {
              eprintln!("\t…cannot write SWF; aborting");
              break;
            } else {
              println!("\t…written to {}", dest_path);
            }
          }

          Err(e) => {
            eprintln!("cannot create {}: {}; aborting", dest_path, e);
            break;
          }
        }
      }

      Err(e) => {
        eprintln!("  error: {}", e);
      }
    }
  }
}
