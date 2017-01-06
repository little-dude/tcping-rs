// use std::io::{Write, Read};
use std::net::{TcpStream, SocketAddrV4};
use std::thread::sleep;
use std::time::Duration;
use std::str;
use std::fmt;
use std::cell::RefCell;

use errors::*;

pub enum Mode {
    Reconnect,
    KeepAlive,
}
pub struct Client {
    server: SocketAddrV4,
    stream: Option<RefCell<TcpStream>>,
    interval: u64,
    count: Option<u32>,
    // timeout: Option<u32>,
    // request_id: u64,
    connection_id: u64,
    mode: Mode,
}

impl fmt::Display for Client {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.stream {
            Some(ref stream) => {
                match stream.borrow().local_addr() {
                    Ok(addr) => write!(f, "{}", addr),
                    Err(_) => write!(f, "client (not connected)"),
                }
            }
            None => write!(f, "client (not connected)"),
        }
    }
}

impl Client {
    pub fn new(server: SocketAddrV4,
               interval: f32,
               count: Option<u32>,
               timeout: Option<u32>,
               reconnect: bool)
               -> Self {
        let mut interval = interval;
        if interval < 0.0001 {
            interval = 0.0001;
        }
        let mode = if reconnect {
            Mode::Reconnect
        } else {
            Mode::KeepAlive
        };
        Client {
            server: server,
            stream: None,
            interval: (interval * 1000.) as u64,
            count: count,
            // timeout: timeout,
            // request_id: 0,
            connection_id: 0,
            mode: mode,
        }
    }

    fn connect(&mut self) -> Result<()> {
        self.connection_id += 1;
        // FIXME: this blocks, potentially much longer than self.interval, making
        // self.interval quite meaningless. Maybe see if we can work something out with
        // TcpStream.set_nonblocking
        // (https://doc.rust-lang.org/std/net/struct.TcpStream.html#method.set_nonblocking)
        let stream = TcpStream::connect(self.server).chain_err(|| ErrorKind::ConnectionFailed)?;
        if let Mode::KeepAlive = self.mode {
            stream.set_read_timeout(Some(Duration::from_millis(100)))
                .chain_err(|| ErrorKind::ConnectionFailed)?;
            stream.set_write_timeout(Some(Duration::from_millis(100)))
                .chain_err(|| ErrorKind::ConnectionFailed)?;
        }
        self.stream = Some(RefCell::new(stream));
        Ok(())
    }

    pub fn run(&mut self) {
        match self.mode {
            Mode::Reconnect => {
                let mut failed = 0;
                let mut success = 0;
                loop {
                    match self.connect() {
                        Ok(_) => {
                            success += 1;
                            println!("{} >>> {} connection successful ({})",
                                     self,
                                     self.server,
                                     self.connection_id);
                        }
                        Err(e) => {
                            failed += 1;
                            println!("connection to {} failed: {}", self.server, e);
                            for e in e.iter().skip(1) {
                                println!("    caused by: {}", e);
                            }
                        }
                    }
                    sleep(Duration::from_millis(self.interval));
                    if let Some(count) = self.count {
                        if self.connection_id >= count as u64 {
                            break;
                        }
                    }
                }
                println!("success: {}, failed {}", success, failed);
            }
            Mode::KeepAlive => unimplemented!(),
        }
    }

    // fn get_echo_response(&mut self) -> Result<()> {
    //     let mut read_buf: [u8; 128] = [0; 128];
    //     if let Some(ref stream) = self.stream {
    //         let num_bytes = stream.borrow_mut().read(&mut read_buf)?;
    //         if num_bytes > 0 {
    //             println!("{} <<< {}: {}", self, self.server, str::from_utf8(&read_buf[0..num_bytes]).unwrap());
    //             Ok(())
    //         } else {
    //             bail!(ErrorKind::ConnectionClosed);
    //         }
    //     } else {
    //         bail!("Cannot read response: no stream open");
    //     }
    // }

    // fn echo_request(&mut self) -> Result<()> {
    //     self.request_id += 1;
    //     match self.stream {
    //         Some(ref stream) => {
    //             stream.borrow_mut().write(format!("ping {}", self.request_id).as_bytes())?;
    //             println!("{} >>> {} : echo request {}", self, self.server, self.request_id);
    //             Ok(())
    //         },
    //         None => {
    //             panic!("Internal error: client does not have a stream open")
    //         },
    //     }
    // }
}
