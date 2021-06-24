//! Example of a server using Unix domain socket. q client can connect with:
//! ```q
//! q)h:hopen `:unix://7000:mattew:oracle
//! ```
//! # Note
//! - A file path to a credential file must be set on `KDBPLUS_ACCOUNT_FILE`. See the README or the module document for the detail of the file format.
//! - You can set an environmental variable `QUDSPATH`to change the default abstract namespace. See the README or the module document for the detail.

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