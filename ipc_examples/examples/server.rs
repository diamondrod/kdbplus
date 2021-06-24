//! Example of a server using Unix domain socket. q client can connect with:
//! ```q
//! q)h:hopen `:unix://7000:mattew:oracle
//! ```
//! # Note
//! - A file path to a credential file must be set on `KDBPLUS_ACCOUNT_FILE`. See the README for a detail of format.
//! - A socket file path must be set on `QUDSPATH`.

use std::io;
use kdbplus::ipc::*;

#[tokio::main]
async fn main() -> io::Result<()>{

  // Start listenening over UDS at the port 7000.
  while let Ok(mut socket) = QStream::accept(ConnectionMethod::UDS, "", 7000).await{
    tokio::task::spawn(async move {
      loop{
        match socket.receive_message().await{
          Ok((_, message)) => {
            println!("request: {}", message);
          },
          _ => {
            socket.shutdown().await.unwrap();
            break
          }
        }
      }
    });
  }

  Ok(())
}