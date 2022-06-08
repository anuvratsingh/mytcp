use tun_tap::{Iface, Mode};

fn main() {
    let nic = Iface::new("tun0", Mode::Tun).expect("Failed to create a TUN device");
    let mut buf = [0u8; 1504];
    let n_bytes = nic.recv(&mut buf[..]).unwrap();
    eprintln!("read {} bytes: {:x?}", n_bytes, &buf[..n_bytes]);
}
