use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::io::Write;
use std::net::TcpStream;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Device {
    device_type: String,
    os: String,
    app_version: String,
    ip: String,
}

impl Device {
    pub fn get_device(
        stream: &mut TcpStream,
        senders: Arc<Mutex<Vec<Device>>>,
        tx: Sender<Device>,
    ) {
        let client_addr = stream.peer_addr().unwrap();
        println!("New connection from: {}", client_addr);

        let mut buffer = [0; 512];
        let bytes_read = stream.peek(&mut buffer).unwrap();
        let device_info: Value = serde_json::from_slice(&buffer[..bytes_read]).unwrap();

        let device = Device {
            device_type: device_info["device_type"].as_str().unwrap().to_string(),
            os: device_info["os"].as_str().unwrap().to_string(),
            app_version: device_info["app_version"].as_str().unwrap().to_string(),
            ip: client_addr.to_string(),
        };

        tx.send(device.clone()).unwrap();
        println!("Device info: {:?}", device);

        stream.write_all(&buffer[..bytes_read]).unwrap();
        println!("Connection closed by: {}", client_addr);
    }
}
