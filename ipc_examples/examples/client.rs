use kdbplus::ipc::*;
use kdbplus::qattribute;

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() -> Result<()> {
    // Connect to qprocess running on localhost:5000 via UDS
    let mut socket = QStream::connect(ConnectionMethod::UDS, "", 5000_u16, "ideal:person").await?;
    println!("Connection type: {}", socket.get_connection_type());

    // Set remote function with an asynchronous text form message
    socket.send_async_message(&"collatz:{[n] seq:enlist n; while[not n = 1; seq,: n:$[n mod 2; 1 + 3 * n; `long$n % 2]]; seq}").await?;

    // Send a text form message synchronously
    let mut result = socket.send_sync_message(&"collatz[12]").await?;
    println!("collatz[12]: {}", result);

    result = socket.send_sync_message(&"collatz[`a]").await?;
    println!("collatz[`a]: {}", result);

    // Send a functional form message synchronously.
    let mut message = K::new_compound_list(vec![
        K::new_symbol(String::from("collatz")),
        K::new_long(100),
    ]);
    result = socket.send_sync_message(&message).await?;
    println!("collatz[100]: {}", result);

    // Modify the query to (`collatz; 20)
    message.pop().unwrap();
    message.push(&K::new_long(20)).unwrap();
    result = socket.send_sync_message(&message).await?;
    println!("collatz[20]: {}", result);

    // Send a functional form message asynchronous.
    message = K::new_compound_list(vec![
        K::new_string(String::from("show"), qattribute::NONE),
        K::new_symbol(String::from("goodbye")),
    ]);
    socket.send_async_message(&message).await?;

    socket.shutdown().await?;

    Ok(())
}
