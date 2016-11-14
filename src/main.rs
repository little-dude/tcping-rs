extern crate clap;

use std::io::{Write, Read, ErrorKind, Error};
use std::net::{TcpListener, TcpStream, SocketAddrV4};
use std::thread::{spawn, JoinHandle, sleep};
use std::time::Duration;
use std::str;
use clap::{Arg, App};


fn main() {
    let matches = App::new("Simple TCP echo server")
        .version("1.0")
        .author("Corentin H. <corentinhenry@gmail.com>")
        .about("Quickly test TCP connections")
        .arg(Arg::with_name("server")
            .short("s")
            .long("server")
            .number_of_values(1)
            .multiple(true)
            .help("Start a server listen on the specified address")
            .takes_value(true))
        .arg(Arg::with_name("client")
            .short("c")
            .long("client")
            .number_of_values(1)
            .multiple(true)
            .help("Start a client on the specified address")
            .takes_value(true))
        .get_matches();

    let mut handles = Vec::<JoinHandle<()>>::new();
    if let Some(addresses) = matches.values_of("server") {
        let mut servers: Vec<SocketAddrV4> = vec![];
        for addr in addresses {
            servers.push(addr.trim().parse().expect("not a value port number"));
        }

        for address in servers.into_iter() {
            handles.push(spawn(move || listen(address)));
        }
    }

    if let Some(addresses) = matches.values_of("client") {
        let mut clients: Vec<SocketAddrV4> = vec![];
        for addr in addresses {
            clients.push(addr.trim().parse().expect("not a value port number"));
        }

        for address in clients.into_iter() {
            handles.push(spawn(move || ping(address)));
        }
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

fn get_client_error_string(error: Error) -> &'static str {
    match error.kind() {
        ErrorKind::ConnectionRefused => "Connection refused",
        ErrorKind::ConnectionAborted => "Connection aborted",
        ErrorKind::ConnectionReset => "Connection reset",
        _ => "Unhandled error",
    }
}

fn ping(server: SocketAddrV4) {
    let mut i: u64 = 0;
    println!("Establishing session to {}", server);
    loop {
        match TcpStream::connect(server) {
            Err(e) => {
                println!("Connection to {} failed: {}",
                         server,
                         get_client_error_string(e));
                i += 1;
                sleep(Duration::from_secs(1));
            }
            Ok(mut stream) => {
                let client = stream.local_addr().unwrap();
                stream.set_read_timeout(Some(Duration::from_millis(100))).unwrap();
                stream.set_write_timeout(Some(Duration::from_millis(100))).unwrap();
                let mut read_buf: [u8; 128] = [0; 128];
                loop {
                    // FIXME: do we really need to allocate a string here? I could not find how to
                    // do this directly with byte literals.
                    let msg = format!("ping {}", i);
                    i += 1;
                    match stream.write(msg.as_bytes()) {
                        Err(e) => {
                            println!("{}: failed to write ping message for {} ({})",
                                     client,
                                     server,
                                     get_client_error_string(e));
                            sleep(Duration::from_secs(1));
                            break;
                        }
                        Ok(_) => {
                            println!("{} >>> {}: {}", client, server, msg);
                            match stream.read(&mut read_buf) {
                                Ok(n) => {
                                    if n > 0 {
                                        println!("{} <<< {}: {}",
                                                 client,
                                                 server,
                                                 str::from_utf8(&read_buf[0..n]).unwrap());
                                    } else {
                                        println!("{}: connection with {} closed (received EOF)",
                                                 client,
                                                 server);
                                        sleep(Duration::from_secs(1));
                                        break;
                                    }
                                    sleep(Duration::from_secs(1));
                                }
                                Err(e) => {
                                    println!("{}: failed to read ping response from {} ({})",
                                             client,
                                             server,
                                             get_client_error_string(e));
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn listen(address: SocketAddrV4) {
    let listener = TcpListener::bind(&address).unwrap();
    println!("listening started on {}, ready to accept", address);
    for stream in listener.incoming() {
        spawn(|| {
            handle_connection(stream.unwrap());
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let server = stream.local_addr().unwrap();
    let peer = stream.peer_addr().unwrap();
    println!("{}: connection established from {}", server, peer);

    // We'll wait for 100 milliseconds to receive a message Concretely, a ErrorKind::WouldBlock
    // io::Error is returned after 100ms if there is nothing to read.
    stream.set_read_timeout(Some(Duration::from_millis(100))).unwrap();

    // We'll wait for 100 milliseconds to send a message
    stream.set_write_timeout(Some(Duration::from_millis(100))).unwrap();

    let mut read_buf: [u8; 128] = [0; 128];
    loop {
        match stream.read(&mut read_buf) {
            Ok(n) => {
                if n > 0 {
                    handle_message(&read_buf, n, &mut stream);
                } else {
                    // according to rust docs[1], n == 0 can indicate one of two scenarios:
                    //
                    // 1. This reader has reached its "end of file" and will likely no longer be
                    //    able to produce bytes. Note that this does not mean that the reader will
                    //    always no longer be able to produce bytes.
                    //
                    // 2. The buffer specified was 0 bytes in length.
                    //
                    // https://doc.rust-lang.org/std/io/trait.Read.html#tymethod.read
                    //
                    // We assume the first scenario since we know our buffer length in not 0.
                    // This means the connection is closed and we can exit.
                    println!("{}: connection with {} is closed (received EOF).",
                             server,
                             peer);
                    return;
                }
            }
            Err(e) => {
                match e.kind() {
                    // We are using a non-blocking socket and there was nothing to read, so Linux
                    // return an EAGAIN error which translates in rust into a "WouldBlock" error.
                    ErrorKind::WouldBlock => continue,
                    _ => {
                        panic!("{}: An unknown error occured while reading the socket for {}: {}",
                               server,
                               peer,
                               e)
                    }
                }
            }
        }
    }
}

fn handle_message(buffer: &[u8], length: usize, stream: &mut TcpStream) {
    let server = stream.local_addr().unwrap();
    let peer = stream.peer_addr().unwrap();
    if let Ok(msg) = str::from_utf8(&buffer[0..length]) {
        println!("{} <<< {} {}", server, peer, msg);
        match stream.write(&buffer[0..length]) {
            Ok(n) => {
                if length < n {
                    panic!("*** {}: Incomplete answer: {} bytes answered instead of {}",
                           peer,
                           n,
                           length);
                } else {
                    println!("{} >>> {} {}", server, peer, msg);
                }
            }
            Err(e) => {
                panic!("*** {}: {:?}", peer, e);
            }
        }
    } else {
        println!("<<< {} {:?}", peer, buffer);
        println!("Received message is not a valid UTF-8 sequence. Ignoring");
    }
}
