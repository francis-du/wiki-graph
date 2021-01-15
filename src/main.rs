use log::{debug, info};
use structopt::StructOpt;

use common::network;

use crate::common::logger::Logger;

mod api;
mod app;
mod common;
mod handler;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "
░██╗░░░░░░░██╗██╗██╗░░██╗██╗  ░██████╗░██████╗░░█████╗░██████╗░██╗░░██╗
░██║░░██╗░░██║██║██║░██╔╝██║  ██╔════╝░██╔══██╗██╔══██╗██╔══██╗██║░░██║
░╚██╗████╗██╔╝██║█████═╝░██║  ██║░░██╗░██████╔╝███████║██████╔╝███████║
░░████╔═████║░██║██╔═██╗░██║  ██║░░╚██╗██╔══██╗██╔══██║██╔═══╝░██╔══██║
░░╚██╔╝░╚██╔╝░██║██║░╚██╗██║  ╚██████╔╝██║░░██║██║░░██║██║░░░░░██║░░██║
░░░╚═╝░░░╚═╝░░╚═╝╚═╝░░╚═╝╚═╝  ░╚═════╝░╚═╝░░╚═╝╚═╝░░╚═╝╚═╝░░░░░╚═╝░░╚═╝\n
Version:",
    about = "A wiki graph app for Logseq Remote Interview."
)]
struct Opts {
    /// Activate debug mode
    #[structopt(short, long)]
    debug: bool,

    /// Set host
    #[structopt(short, long, default_value = "0.0.0.0")]
    host: String,

    /// Set port
    #[structopt(short, long, default_value = "3690")]
    port: String,

    /// Using proxy network,`PROXY=https://127.0.0.1:1080`
    #[structopt(long)]
    proxy: bool,

    /// subcommands
    #[structopt(subcommand)]
    subcmd: SubCommand,
}

#[derive(StructOpt, Debug)]
enum SubCommand {
    /// start a app service
    App,
    /// start a api service
    Api,
}

/// main.css
#[async_std::main]
async fn main() -> tide::Result<()> {
    let opts: Opts = Opts::from_args();

    if opts.proxy {
        network::set_proxy()?;
    }

    match opts.debug {
        true => {
            match Logger::default().level("debug").setup() {
                Ok(logger) => {
                    debug!("CLI {:?}", opts);
                    debug!("Log path is [{}]", logger.log_path.display());
                }
                Err(err) => println!("Logger setup error! \n{}", err),
            };
        }
        false => {
            match Logger::default().level("info").setup() {
                Ok(logger) => {
                    info!("Log path is [{}]", logger.log_path.display());
                }
                Err(err) => println!("Logger setup error! \n{}", err),
            };
        }
    }

    match opts.subcmd {
        SubCommand::App => {
            app::APP::new()
                .host(opts.host)
                .port(opts.port)
                .start()
                .await?;
        }
        SubCommand::Api => {
            api::API::new()
                .host(opts.host)
                .port(opts.port)
                .start()
                .await?;
        }
    }
    Ok(())
}
