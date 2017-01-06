use clap::{Arg, App, SubCommand, ArgMatches};

pub fn get_args<'a>() -> ArgMatches<'a> {
    App::new("tcping")
        .version("1.0")
        .author("Corentin H. <corentinhenry@gmail.com>")
        .about("Quickly test TCP connections")
        .subcommand(SubCommand::with_name("server")
            .arg(Arg::with_name("address").long("address")
                         // this option can be specified multiple time, each time with one value
                         .number_of_values(1).multiple(true)
                         // this option must be provided at least once
                         .required(true)
                         .help("Specify an address to listen to"))
            .arg(Arg::with_name("connection_count")
                .long("connection-count")
                .number_of_values(1)
                .help("Number of connections to accept before exiting. If not specified, the \
                       server keeps accepting connections."))
            .arg(Arg::with_name("replies_count")
                .long("replies-count")
                .number_of_values(1)
                .help("Number of connections to accept before exiting. If not specified, the \
                       server keeps accepting connections."))
            .arg(Arg::with_name("timeout")
                .long("timeout")
                .number_of_values(1)
                .help("Tell the server to exit after the timeout, in seconds. If not specified, \
                       the server keeps running.")))
        .subcommand(SubCommand::with_name("client")
            .arg(Arg::with_name("address")
                .long("address")
                .number_of_values(1)
                .number_of_values(1)
                .required(true)
                .help("Specify the address to connect to"))
            .arg(Arg::with_name("count")
                .long("count")
                .number_of_values(1)
                .help("Number of echo requests to send"))
            .arg(Arg::with_name("timeout")
                .long("timeout")
                .number_of_values(1)
                .help("Timeout for each echo request"))
            .arg(Arg::with_name("reconnect")
                .long("reconnect")
                .help("If specified, each echo request will use a new TCP connection. \
                       Otherwise, the same TCP connection is reused."))
            .arg(Arg::with_name("interval")
                .long("interval")
                .default_value("1")
                .number_of_values(1)
                .help("Interval between each echo request. This is not very precise, especially \
                       for small values, because we wait for a request to be answered before \
                       sending another one, and we don't take this time into account. This may \
                       be improved in the future.")))
        .get_matches()
}
