use reqwest::blocking::{Client, Response};
use std::{fs::File, process::exit};
use std::{io::Write as _, path::PathBuf};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct CLIOpt {
  #[structopt(
    long,
    default_value = "0",
    help = "The index of the first loop to scrap."
  )]
  from: usize,
  #[structopt(
    long,
    default_value = "7911",
    help = "The index of the last loop to scrap."
  )]
  to: usize,
  #[structopt(short = "d", long, help = "Where to put downloaded loops.")]
  dest_dir: Option<PathBuf>,
}

fn main() {
  let cli_opt = CLIOpt::from_args();
  let client = Client::new();

  let dest_dir = cli_opt.dest_dir.or(dirs::download_dir());
  if dest_dir.is_none() {
    eprintln!("destination directory is unknown; please provide one with the --dest-dir switch");
    exit(1);
  }
  let dest_dir = dest_dir.unwrap();

  for i in cli_opt.from..cli_opt.to {
    let url = format!("https://z0r.de/L/z0r-de_{}.swf", i);

    println!("scrapping loop {} ({})…", i, url);
    match client.get(&url).send().and_then(Response::bytes) {
      Ok(swf_bytes) => {
        println!("\t…fetched");
        let dest_path = format!("{}/loop_{}.swf", dest_dir.display(), i);

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
        break;
      }
    }
  }
}
