use etherparse::{Ipv4HeaderSlice, TcpHeaderSlice};
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
                let source = p.source_addr();
                let destination = p.destination_addr();
                let _protocol = p.protocol();

                match TcpHeaderSlice::from_slice(&buf[4 + p.slice().len()..]) {
                    Ok(p) => {
                        eprintln!(
                            "{} -> {} {}b of tcp to port {}",
                            source,
                            destination,
                            p.slice().len(),
                            p.destination_port()
                        );
                    }
                    Err(e) => eprintln!("Unknown TCP packet {}", e),
                }
            }
            Err(e) => eprintln!("Unknown packet {}", e),
        }
    }
}
