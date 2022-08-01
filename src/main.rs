fn main() {
    let ports = serialport::available_ports().expect("No ports found!");
    // println!("Available ports: {:?}", ports);
    println!("Available ports:");
    for p in ports {
        println!("{}", p.port_name);
    }
}
