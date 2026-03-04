use std::net::UdpSocket;
use std::io::{self, stdin};

// Simple function that asks the user for two 1-D vectors (lengths entered separately),
// asks what the cells shall contain (either manual entry or a single fill value),
// and returns a Vec<u8> ready to send over UDP.
// Format sent: [len_a (i32 BE)][len_b (i32 BE)][elements of A (i32 BE)...][elements of B (i32 BE)...]
// Minimal parsing/validation so it stays simple and compiles.
fn get_vectors_and_convert_to_bytes() -> Vec<u8> {
    let mut input = String::new();

    println!("Length of vectors:");
    input.clear();
    stdin().read_line(&mut input).expect("read failed");
    let len: u8 = input.trim().parse().unwrap_or(0);


    // helper to read a vector: either manual entry or fill with one value
    fn read_vector(len: u8, name: &str) -> Vec<u8> {
        let mut vec: Vec<u8> = Vec::with_capacity(len.max(0) as usize);
        let mut input = String::new();
        println!("For {}: enter any key + Enter to fill cell", name);

        for i in 0..len {
            input.clear();
            print!("{}[{}]: ", name, i + 1);
            io::Write::flush(&mut io::stdout()).ok();
            stdin().read_line(&mut input).expect("read failed");
            let v = input.trim().parse().unwrap_or(0);
            vec.push(v);
        }

        vec
    }

    let vec_a = read_vector(len, "A");
    let vec_b = read_vector(len, "B");

    let mut bytes: Vec<u8> = Vec::new();
    bytes.extend_from_slice(&len.to_be_bytes());

    for v in vec_a {
        bytes.extend_from_slice(&v.to_be_bytes());
    }
    for v in vec_b {
        bytes.extend_from_slice(&v.to_be_bytes());
    }

    bytes
}

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;

    println!("Prepare two vectors to send to server (Ctrl+C to exit)");
    let packet = get_vectors_and_convert_to_bytes();
    // send to localhost:3400 by default
    socket.send_to(&packet, "127.0.0.1:3400")?;
    println!("Sent {} bytes", packet.len());

    let mut buf = [0;1000];
    let (amt, src) = socket.recv_from(&mut buf)?;
    let buf = &mut buf[..amt];

    print!("client: received dot product: {:?}", buf);



    Ok(())

}