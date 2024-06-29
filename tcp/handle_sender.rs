use std::{io::Write, net::TcpStream};

pub fn handle_sender(stream: &mut TcpStream, buffer: &mut [u8]) -> Result<(), std::io::Error> {
    println!(
        "Receiver function called with: {}",
        String::from_utf8_lossy(buffer)
    );

    stream.write_all(b"You are the Sender for this session\n")?;
    stream.flush()?;

    Ok(())
}
