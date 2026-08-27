use std::net::TcpStream;
use tokio::time::{self, Duration};
use btclib::network::Message;

async fn handle_stream(stream: &TcpStream) {
    
}

#[tokio::main]
async fn main(){
    let stream = TcpStream::connect("127.0.0.1:8333").unwrap();
    handle_stream(&stream).await;
}
