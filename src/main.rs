use futures::future::TryFutureExt as _;
use reqwest::{Client, Response};
use std::sync::mpsc::channel;
use std::sync::Arc;
use structopt::StructOpt;
use tokio::fs::File;
use tokio::io::AsyncWriteExt as _;
use tokio::spawn;

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
  #[structopt(
    short = "d",
    long,
    default_value = "/tmp",
    help = "Where to put downloaded loops."
  )]
  dest_dir: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let cli_opt = Arc::new(CLIOpt::from_args());
  let client = Arc::new(Client::new());
  let (control_flow_sx, control_flow_rx) = channel();

  for i in cli_opt.from..cli_opt.to {
    if control_flow_rx.try_recv().is_ok() {
      break;
    }

    let cli_opt = cli_opt.clone();
    let client = client.clone();
    let control_flow_sx = control_flow_sx.clone();
    spawn(async move {
      let url = format!("https://z0r.de/L/z0r-de_{}.swf", i);

      println!("scrapping loop {} ({})…", i, url);
      match client.get(&url).send().and_then(Response::bytes).await {
        Ok(swf_bytes) => {
          println!("\t…fetched");
          let dest_path = format!("{}/loop_{}.swf", cli_opt.dest_dir, i);

          match File::create(&dest_path).await {
            Ok(mut file) => {
              let write = file.write_all(&swf_bytes).await;
              if write.is_err() {
                eprintln!("\t…cannot write SWF; aborting");
                control_flow_sx.send(()).unwrap();
              } else {
                println!("\t…written to {}", dest_path);
              }
            }

            Err(e) => {
              eprintln!("cannot create {}: {}; aborting", dest_path, e);
              control_flow_sx.send(()).unwrap();
            }
          }
        }

        Err(e) => {
          eprintln!("  error: {}", e);
          control_flow_sx.send(()).unwrap();
        }
      }
    });
  }

  Ok(())
}
