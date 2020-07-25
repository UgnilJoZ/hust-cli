use std::net::UdpSocket;
use std::time::{Duration, Instant};
use std::io::{Error, ErrorKind};
use std::collections::HashSet;
use std::iter::FromIterator;

const DISCOVERY_TEXT: &[u8] = b"M-SEARCH * HTTP/1.1
HOST: 239.255.255.250:1900
MAN: ssdp:discover
MX: 10
ST: ssdp:all

";

fn receive_answer(socket: &UdpSocket) -> std::io::Result<String> {
    let mut buf = [0; 8192];
    let (answer_size, _) = socket.recv_from(&mut buf)?;
    let answer = String::from_utf8_lossy(&buf[0..answer_size]);
    let mut answer_lines = answer.lines();
    if let Some(firstline) = answer_lines.next() {
        if ! firstline.starts_with("HTTP/1.1 200 OK") {
            return Err(Error::from(ErrorKind::InvalidData))
        }
        for line in answer_lines {
            if let Some(url) = line.strip_prefix("LOCATION: ") {
                return Ok(String::from(url))
            }
        }
    }
    Err(Error::from(ErrorKind::InvalidData))
}

/// Return a list of Hue bridge URLs we can find in the network within `timeout`.
pub fn discover(max_devices: usize, timeout: Duration) -> std::io::Result<Vec<String>> {
    let start = Instant::now();
    let mut time_spend = Duration::zero();
    let mut urls = Vec::new();
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.send_to(DISCOVERY_TEXT, "239.255.255.250:1900")?;
    while (time_spend < timeout) && (urls.len() < max_devices) {
        socket.set_read_timeout(Some(timeout - time_spend))?;
        match receive_answer(&socket) {
            Ok(url) => urls.push(url),
            Err(e) => {
				if e.kind() == ErrorKind::WouldBlock {
					break
				} else {
					eprintln!(" => {}", e)
				}
			}
        }
		time_spend = Instant::now() - start;
    }
    // Make entries unique
	urls = HashSet::<String>::from_iter(urls).into_iter().collect();
	// We managed it
    Ok(urls)
}