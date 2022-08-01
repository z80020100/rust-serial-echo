use std::time::Duration;
use std::str;
use std::io;

fn main() {
    let ports = serialport::available_ports().expect("No ports found!");
    // println!("Available ports: {:?}", ports);
    println!("Available ports:");
    for p in ports {
        println!("{}", p.port_name);
    }
    println!("Open: /dev/ttyS0");
    let mut port = serialport::new("/dev/ttyS0", 115200)
        .timeout(Duration::from_millis(10))
        .open().expect("Failed to open port");
    let version = "rust-serial-echo v0.1.0\r\n".as_bytes();
    port.write(version).expect("Write failed!");
    println!("Echo server started!");
    let mut serial_buf: Vec<u8> = vec![0; 1024];
    loop {
        match port.read(serial_buf.as_mut_slice()) {
            Ok(length) => {
                let data = match str::from_utf8(&serial_buf) {
                    Ok(v) => v,
                    Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                };
                println!("Received data: {}, length: {}", data, length);
                port.write(&serial_buf[0..length]).expect("Write failed!");
            },
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
            Err(e) => eprintln!("{:?}", e),
        }
    }
}
