use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addr = "0.0.0.0:60010";
    let listener = TcpListener::bind(addr).await?;
    println!("Listening on: {}", addr);

    loop {
        // Accept new request
        let (mut stream, _) = listener.accept().await?;
        println!("New client connection");

        // Handle new echo request
        tokio::spawn(async move {
            let mut buf = vec![0; 1024];

            // In a loop, read data from the socket and write the data back.
            loop {
                let n = stream
                    .read(&mut buf)
                    .await
                    .expect("failed to read data from socket");
                println!("Send back {:?} bytes to client", n);

                if n == 0 {
                    return;
                }

                stream
                    .write_all(&buf[0..n])
                    .await
                    .expect("failed to write data to socket");
            }
        });
    }
}
