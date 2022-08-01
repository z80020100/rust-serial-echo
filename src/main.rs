use std::time::Duration;

fn main() {
    let ports = serialport::available_ports().expect("No ports found!");
    // println!("Available ports: {:?}", ports);
    println!("Available ports:");
    for p in ports {
        println!("{}", p.port_name);
    }
    let mut port = serialport::new("/dev/ttyS0", 115200)
        .timeout(Duration::from_millis(10))
        .open().expect("Failed to open port");
    let output = "rust-serial-echo v0.1.0\r\n".as_bytes();
    port.write(output).expect("Write failed!");
}
