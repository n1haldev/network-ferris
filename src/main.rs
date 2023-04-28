use pcap;
use std::string::String;
fn main() {
    let device = pcap::Device::lookup()
        .expect("Device lookup failed!")
        .expect("No device available");
    println!("Using device {}", device.name);

    let mut cap = pcap::Capture::from_device(device)
        .unwrap()
        .immediate_mode(true)
        .open()
        .unwrap();
    
    let mut data_used : u128 = 0;
    loop {
        match cap.next_packet() {
            Ok(packet) => { 
                data_used += packet.header.len as u128;
                println!("{}", data_used);
                // println!("Packet: {}", packet.header.len);
            }
            Err(e) => {
                println!("Error capturing the packet!");
                break;
            }
        }
    }
}
