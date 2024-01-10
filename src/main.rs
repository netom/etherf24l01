use tun_tap::{Iface, Mode};

fn main() {
    let iface = Iface::new("mytap", Mode::Tap).expect("Failed to create a TAP device");
    //let name = iface.name();
    // Configure the device ‒ set IP address on it, bring it up.
    let mut buffer = vec![0; 10240]; // Even jumbo frames fit
    loop {
        let n = iface.recv(&mut buffer).unwrap();
        println!("Got {} bytes: \n {:?}", n, buffer.get(0..n));
    }
}
