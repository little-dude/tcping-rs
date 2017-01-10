use std::io::{Write, Read};
use std::net::{TcpListener, TcpStream, SocketAddrV4};
use std::thread::{spawn, JoinHandle};
use std::time::Duration;
use std::str;
use std::sync::{Arc, Mutex};


pub struct Server {
    address: SocketAddrV4,
    expected_connections: Option<u32>,
    expected_echo_replies: Option<u32>,
}

impl Server {
    pub fn new(address: SocketAddrV4,
               expected_connections: Option<u32>,
               expected_replies: Option<u32>)
               -> Self {
        Server {
            address: address,
            expected_connections: expected_connections,
            expected_echo_replies: expected_replies,
        }
    }

    pub fn listen(&self) {
        match TcpListener::bind(&self.address) {
            Ok(listener) => {
                println!("listening started on {}, ready to accept", self.address);
                let replies_count = Arc::new(Mutex::new(0));
                let mut handles = Vec::<JoinHandle<()>>::new();
                for (connection_count, stream) in listener.incoming().enumerate() {
                    let expected_echo_replies = self.expected_echo_replies;
                    let count = replies_count.clone();
                    handles.push(spawn(move || {
                        handle_connection(stream.unwrap(), count, expected_echo_replies);
                    }));
                    if let Some(expected_connections) = self.expected_connections {
                        if connection_count as u32 == expected_connections {
                            println!("Got {} connections. Exiting", connection_count as u32);
                            // should we kill the active threads first?
                            // I think that's taken care of when the process exits but maybe it's
                            // cleaner?
                            break;
                        }
                    }
                }
                for handle in handles {
                    handle.join().unwrap();
                }
            }
            Err(e) => {
                println!("Cannot listen on {}", self.address);
                println!("    caused by: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream,
                     replies_count: Arc<Mutex<u32>>,
                     expected_replies: Option<u32>) {
    let server = stream.local_addr().unwrap();
    let peer = stream.peer_addr().unwrap();
    println!("{}: connection established from {}", server, peer);
    // We'll wait for 100 milliseconds to receive a message.
    // An io:Error (ErrorKind::WouldBlock) is returned after 100ms if there is nothing to read.
    stream.set_read_timeout(Some(Duration::from_millis(100))).unwrap();

    // We'll wait for 100 milliseconds to send a message
    stream.set_write_timeout(Some(Duration::from_millis(100))).unwrap();

    let mut read_buf: [u8; 128] = [0; 128];
    loop {
        match stream.read(&mut read_buf) {
            Ok(n) => {
                if n > 0 {
                    handle_message(&read_buf, n, &mut stream, &replies_count);
                    if let Some(expected_replies) = expected_replies {
                        if *replies_count.lock().unwrap() >= expected_replies {
                            return;
                        }
                    }
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
                    ::std::io::ErrorKind::WouldBlock => continue,
                    _ => {
                        // FIXME: should bail
                        panic!();
                    }
                }
            }
        }
    }
}

pub fn handle_message(buffer: &[u8],
                      length: usize,
                      stream: &mut TcpStream,
                      replies_count: &Arc<Mutex<u32>>) {
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
                    *replies_count.lock().unwrap() += 1;
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
