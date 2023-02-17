use crate::config::CONNECTION_RETRIES;
use crate::config::IP_SERVERS;
use std::io::{BufRead, BufReader, Write};
use std::io::{Error, ErrorKind};
use std::net::TcpStream;
use std::time;

fn connect_to_server(retries: u16) -> Result<TcpStream, std::io::Error> {
    for _retrie in 0..retries {
        if let Ok(stream_server) = TcpStream::connect(IP_SERVERS) {
            return Ok(stream_server);
        } else {
            let sleep_time = time::Duration::from_millis(2000);
            std::thread::sleep(sleep_time);
        }
    }

    Err(Error::new(
        ErrorKind::Other,
        "Failed to establish connection with web server",
    ))
}

pub fn connect(req: String) -> Result<(), std::io::Error> {
    match connect_to_server(CONNECTION_RETRIES) {
        Ok(mut server) => {
            server.write_all(req.as_bytes())?;
            let mut buf_reader = BufReader::new(&mut server);
            let mut response = String::new();
            let mut status = String::new();

            buf_reader.read_line(&mut status)?;
            loop {
                buf_reader.read_line(&mut response)?;
                if response.ends_with("\r\n\r\n") {
                    break;
                }
            }

            println!("REQUEST STATUS: {}", status);

            if !response.eq("\r\n\r\n") {
                println!("ID\tESTADO\t\tUSUARIO");
                println!("-------------------------------");
                println!("{}", response);
            }
        }
        Err(_) => {
            return Err(Error::new(
                ErrorKind::NotConnected,
                "Could not connect to server.",
            ));
        }
    }

    Ok(())
}
