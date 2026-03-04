
use std::net::UdpSocket;

fn dot_product(package:&mut [u8]) -> i32 {
    let mut input:Vec<u8> = package.to_vec();
    let mut integers:Vec<i32> = input.iter().map(|&x| x as i32).collect();

    let vector_length = integers.remove(0);
    let mut result:i32 = 0;
    for i in 0..vector_length {
        result += integers[i as usize] * integers[(i + vector_length) as usize];
    }
    return result;
}
fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:3400")?;
    {
        // Receives a single datagram message on the socket. If `buf` is too small to hold
        // the message, it will be cut off.
        let mut buf = [0;1000];
        let (amt, src) = socket.recv_from(&mut buf)?;


        // Redeclare `buf` as slice of the received data and send reverse data back to origin.
        let buf = &mut buf[..amt];

        let dot_product = dot_product(buf);

        socket.send_to(&dot_product.to_be_bytes(), src).expect("failed sending");

        print!("Server: received {:?}", buf);



    }
    Ok(())
}

