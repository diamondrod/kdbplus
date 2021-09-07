use kdbplus::ipc::*;
use std::io;
use std::io::Write;

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main()->io::Result<()>{
  let mut socket = QStream::connect(ConnectionMethod::UDS, "", 5000_u16, "kdb:fastest").await?;
  let mut query = String::new();
  let version = socket.send_sync_message(&".z.K").await?;
  let version_date = socket.send_sync_message(&".z.k").await?;
  println!("KDB+ {:.1} {} Console\n", version, version_date);
  loop{
    // Ensure that the buffer is empty 
    query.clear();
    print!("q)");
    io::stdout().flush().unwrap();
    // Read user input
    io::stdin().read_line(&mut query)?;
    // Discard new line
    query.pop();
    match query.as_str(){
      "\\\\" => break,
      _ => {
        let result = socket.send_sync_message(&query.as_str()).await?;
        println!("{}", result);
      }
    }
  }
  Ok(())
}