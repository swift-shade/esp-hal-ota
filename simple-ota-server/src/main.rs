use std::{
    io::{Read, Write},
    net::TcpListener,
};

fn main() {
    let binary = std::fs::read("./firmware.bin").unwrap();
    let mut binary_bytes = Vec::with_capacity(binary.len() + 4);
    binary_bytes.extend_from_slice(&[255, 255, 255, 255]); // to calc crc32 from binary, append
                                                           // 4 x 255's at the beggining
    binary_bytes.extend_from_slice(&binary);
    let binary_crc = crc32fast::hash(&binary_bytes);

    println!("BINARY_SIZE: {}", binary.len());
    println!("BINARY CRC32: {}", binary_crc);

    let listener = TcpListener::bind("0.0.0.0:6969").unwrap();
    for stream in listener.incoming() {
        println!("Connection");
        let mut stream = stream.unwrap();

        let buf = (binary.len() as u32).to_le_bytes();
        _ = stream.write_all(&buf);

        let chunks = binary.chunks(4096 * 2);
        let mut buf = [0; 1];
        for chunk in chunks {
            println!("Writing: {}", chunk.len());

            _ = stream.write_all(chunk);
            _ = stream.read_exact(&mut buf);

            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    }
}
