use std::{net::TcpListener, net::TcpStream, thread, io::{self, Read, Write}, time};

use immie2d_shared::TestStruct;

fn  handle_sender(mut stream: TcpStream) -> io::Result<()>{
    let mut buf = [0;512];
    for _ in 0..5 {
        let bytes_read = stream.read(&mut buf)?;

        if bytes_read == 0 {
            println!("no bytes read");
            return Ok(());
        }
        stream.write(&buf[..bytes_read]).expect("failed to write"); // TODO add support for client closing connection.

        println!("From the sender: {}", String::from_utf8_lossy(&buf));

        thread::sleep(time::Duration::from_secs(1));
    }
    println!("fully looped");
    stream.shutdown(std::net::Shutdown::Both).expect("failed to shut donw");
    return Ok(());
}

fn main() {
    // bind the server to listen to an address and port
    let receiver_listener = TcpListener::bind("127.0.0.1:7878").expect("Failed to bind to address and port");
    // handle multiple client connections through dynamic vec
    let mut thread_vec: Vec<thread::JoinHandle<()>> = Vec::new();
    // continually iterate through clients attempting to connect
    for stream in receiver_listener.incoming() {
        let stream = stream.expect("failed");
        // for each connection, create a thread and bind the handle function to it
        let handle = thread::spawn(move || {
            handle_sender(stream).unwrap_or_else(|error| eprintln!("[handle_sender thread]: {:?}", error));
        });
        // add the created thread to the vec of threads
        thread_vec.push(handle);
        break; // break to stop accepting connection requests
    }
    
    println!("no longer accepting connection requests");

    for handle in thread_vec {
        // join the threads
        handle.join().unwrap();
    }
}