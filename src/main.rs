use etherparse::Ipv4HeaderSlice;
use tun_tap::{Iface, Mode};

fn main() {
    let nic = Iface::new("tun0", Mode::Tun).expect("Failed to create a TUN device");
    let mut buf = [0u8; 1504];
    loop {
        let n_bytes = nic.recv(&mut buf[..]).unwrap();
        let _eth_flags = u16::from_be_bytes([buf[0], buf[1]]);
        let _eth_proto = u16::from_be_bytes([buf[2], buf[3]]);
       
        match Ipv4HeaderSlice::from_slice(&buf[4..n_bytes]) {
            Ok(p) => {
                eprintln!(
                    "Payload length: {} Protocol: {} Source: {} Destination:{}",
                    p.payload_len(),
                    p.protocol(),
                    p.source_addr(),
                    p.destination_addr()
                );
            }
            Err(e) => eprintln!("Packet {} discarded", e),
        }
    }
}
