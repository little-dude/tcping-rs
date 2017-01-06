#![feature(plugin)]
#![plugin(clippy)]

#![recursion_limit = "1024"]
#[macro_use]
extern crate error_chain;
extern crate clap;
use clap::ArgMatches;
use std::thread::{spawn, JoinHandle};
use std::str;

mod errors;
use errors::*;

mod command;
mod client;
mod server;


quick_main!(run);

fn run() -> Result<()> {
    match command::get_args().subcommand() {
        ("server", Some(server_args)) => run_server(server_args),
        ("client", Some(client_args)) => run_client(client_args),
        _ => {
            bail!(ErrorKind::InvalidInvocation("Invalid sub-command"));
        }
    }

}

fn run_server(server_args: &ArgMatches) -> Result<()> {
    let mut handles = Vec::<JoinHandle<()>>::new();
    // unwrap safely since we marked this option as required
    let addresses = server_args.values_of("address").unwrap();
    let mut servers: Vec<server::Server> = vec![];
    for addr in addresses {
        servers.push(server::Server::new(addr.trim().parse().chain_err(|| "Invalid port number")?,
                                         None,
                                         None));
    }
    for server in servers {
        handles.push(spawn(move || server.listen()));
    }
    for handle in handles {
        // cannot use chain_err here, not sure why
        handle.join().unwrap();
    }
    Ok(())
}

fn run_client(client_args: &ArgMatches) -> Result<()> {
    let address = client_args.value_of("address")
        .unwrap()
        .trim()
        .parse()
        .chain_err(|| "Invalid port number")?;
    let interval = client_args.value_of("interval")
        .unwrap()
        .parse::<f32>()
        .chain_err(|| "Invalid interval")?;

    let mut count: Option<u32> = None;
    if let Some(count_arg) = client_args.value_of("count") {
        count = Some(count_arg.parse::<u32>().chain_err(|| "Invalid count")?);
    }

    let mut timeout: Option<u32> = None;
    if let Some(timeout_arg) = client_args.value_of("timeout") {
        timeout = Some(timeout_arg.parse::<u32>().chain_err(|| "Invalid timeout")?);
    }
    let reconnect = client_args.is_present("reconnect");
    let mut client = client::Client::new(address, interval, count, timeout, reconnect);
    client.run();
    Ok(())
}
