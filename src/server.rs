use tokio;

#[tokio::main]
async fn main() {
    let socket = tokio::net::UdpSocket::bind("127.0.0.1:7777").await.unwrap();
    println!("UDP server listening on 7777");

    let mut buf = [0u8; 1024];
    loop {
        let (n, addr) = socket.recv_from(&mut buf).await.unwrap();
        println!("got {n} bytes from {addr}");
    }
}
