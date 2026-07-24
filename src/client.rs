use tokio;

#[tokio::main]
async fn main() {
    let socket = tokio::net::UdpSocket::bind("127.0.0.1:0").await.unwrap();
    socket
        .send_to(b"hello server", "127.0.0.1:7777")
        .await
        .unwrap();
    println!("sent!");
}
