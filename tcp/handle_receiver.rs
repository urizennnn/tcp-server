use std::{io::Write, net::TcpStream};
pub fn handle_receiver(stream: &mut TcpStream, buffer: &[u8]) -> Result<(), std::io::Error> {
    stream.write_all(b"You are the receiver for this session\n")?;
    stream.flush()?;
    println!("Receiver function handled");

    Ok(())
}
