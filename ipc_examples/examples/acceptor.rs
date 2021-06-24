//! Example of acceptor.
//! # Note
//! A file path to a credential file must be set on `KDBPLUS_ACCOUNT_FILE`. See the README or the module document for the detail of the file format.

use std::io;
use kdbplus::ipc::*;

#[tokio::main]
async fn main() -> io::Result<()>{

  // Start listenening over TCP at the port 7000.
  let mut socket_tcp=QStream::accept(ConnectionMethod::TCP, "127.0.0.1", 7000).await?;

  // Send a query with the socket.
  let greeting=socket_tcp.send_sync_message(&"string `Hello").await?;
  println!("Greeting: {}", greeting);

  socket_tcp.shutdown().await?;

  Ok(())
}