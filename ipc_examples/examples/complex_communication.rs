use kdbplus::ipc::*;

// Print `K` object.
fn print(obj: &K) {
    println!("{}", obj);
}

// Calculate something from two long arguments.
fn nonsense(arg1: i64, arg2: i64) -> i64 {
    arg1 * arg2
}

#[tokio::main]
async fn main() -> Result<()> {
    // Connect to qprocess running on localhost:5000 via TCP
    let mut socket =
        QStream::connect(ConnectionMethod::TCP, "localhost", 5000_u16, "ideal:person").await?;

    // Set a function which sends back a non-response message during its execution.
    socket
        .send_async_message(
            &"complex:{neg[.z.w](`print; \"counter\"); what: .z.w (`nonsense; 1; 2); what*100}",
        )
        .await?;

    // Send a query `(`complex; ::)` without waiting for a response.
    socket
        .send_message(
            &K::new_compound_list(vec![K::new_symbol(String::from("complex")), K::new_null()]),
            qmsg_type::synchronous,
        )
        .await?;

    // Receive an asynchronous call from the function.
    match socket.receive_message().await {
        Ok((qmsg_type::asynchronous, message)) => {
            println!("asynchronous call: {}", message);
            let list = message.as_vec::<K>().unwrap();
            if list[0].get_symbol().unwrap() == "print" {
                print(&list[1])
            }
        }
        _ => unreachable!(),
    }

    // Receive a synchronous call from the function.
    match socket.receive_message().await {
        Ok((qmsg_type::synchronous, message)) => {
            println!("synchronous call: {}", message);
            let list = message.as_vec::<K>().unwrap();
            if list[0].get_symbol().unwrap() == "nonsense" {
                let res = nonsense(list[1].get_long().unwrap(), list[2].get_long().unwrap());
                // Send bach a response.
                socket
                    .send_message(&K::new_long(res), qmsg_type::response)
                    .await?;
            }
        }
        _ => unreachable!(),
    }

    // Receive a final result.
    match socket.receive_message().await {
        Ok((qmsg_type::response, message)) => {
            println!("final: {}", message);
        }
        _ => unreachable!(),
    }

    Ok(())
}
