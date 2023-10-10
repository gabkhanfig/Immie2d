use std::{net::TcpStream, io::{self, Write, BufReader, BufRead, ErrorKind}};
use std::str;


fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:7878").expect("failed to connect");

    for _ in 0..7 {
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("failed to read user input");
        
        // write to the tcp connection
        let stream_write_result = stream.write(user_input.as_bytes());
        if(stream_write_result.is_err()) {
            let err = stream_write_result.unwrap_err();
            if err.kind() == ErrorKind::ConnectionAborted {
                println!("Server aborted connection");
            }
            else {
                println!("Some write error occurred {err}");
            } 
            break;
        }

        let mut reader = BufReader::new(&stream);

        let mut buffer: Vec<u8> = Vec::new();
        let stream_read_result = reader.read_until(b'\n', &mut buffer);
        if(stream_read_result.is_err()) {
            let err = stream_read_result.unwrap_err();
            if err.kind() == ErrorKind::ConnectionAborted {
                println!("Server aborted connection");
            }
            else {
                println!("Some write error occurred {err}");
            }
            break;
        }

        println!("read from server: {}\n", str::from_utf8(&buffer).unwrap());   
    }

    //stream.shutdown(std::net::Shutdown::Both).expect("lmao");
}
