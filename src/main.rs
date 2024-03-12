use tun_tap::{Iface, Mode};

enum SerialMessage<'a> {
    Data(&'a [u8]),
    Ping,
    Pong,
    TxMode,
    RxMode,
}

struct TxFrame {
    advs_rem: u8,
    data: Vec<u8>,
}

fn main() {
    let mut txframes = Vec::<TxFrame>::new();

    let iface = Iface::new("mytap", Mode::Tap).expect("Failed to create a TAP device");
    //let name = iface.name();
    // Configure the device ‒ set IP address on it, bring it up.
    let mut buffer = vec![0; 10240]; // Even jumbo frames fit
    loop {
        let n = iface.recv(&mut buffer).unwrap();
        let data = buffer.get(0..n).unwrap();
        println!("Got {} bytes: \n {:?}", n, data);
    }
}
