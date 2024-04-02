use tun_tap::{Iface, Mode};

struct StationAddress(u8);

struct FrameId(u8);

struct FragmentId(u8);

enum SerialMessage<'a> {
    Data(&'a [u8]),
    Ping,
    Pong,
    TxMode,
    RxMode,
}

enum RadioMessage<'a> {
    Advertisement {
        src: StationAddress,
        dst: StationAddress,
        frame_id: FrameId,
        len: u8,
    },
    Request {
        src: StationAddress,
        dst: StationAddress,
        frame_id: FrameId,
        len: u8,
        mask: u64,
    },
    Data {
        src: StationAddress,
        dst: StationAddress,
        frame_id: FrameId,
        fragment_id: FragmentId,
        data: &'a [u8],
    },
}

struct TxFrame {
    advs_rem: u8,
    data: Vec<u8>,
}

fn main() {
    let mut txframes = Vec::<TxFrame>::new();

    let iface =
        Iface::without_packet_info("nrf24l01", Mode::Tap).expect("Failed to create a TAP device");
    // Configure the device ‒ set IP address on it, bring it up.
    let mut buffer = vec![0; 1518];
    loop {
        let n = iface.recv(&mut buffer).unwrap();
        let data = buffer.get(0..n).unwrap();
        println!("Got {} bytes: \n {:?}", n, data);
        iface.send(data).unwrap();
    }
}
