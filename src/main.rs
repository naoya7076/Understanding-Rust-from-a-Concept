use std::io::Read;
use std::net::TcpListener;
use std::{error::Error, io::Write, net::TcpStream};

fn echo_process(stream: &mut TcpStream) -> Result<(), Box<dyn Error>> {
    let mut buf = [0_u8; 1024];
    loop {
        stream.write_all("input =>".as_bytes())?;
        let read = stream.read(&mut buf);
        match read {
            Ok(0) => break,
            Ok(n) => {
                stream.write_all("output=>".as_bytes())?;
                stream.write_all(&buf[..n])?;
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        panic!("port number is required");
    }
    let port: usize = args[1].parse().expect("port number must be a number");
    let addr = format!("localhost:{}", port);
    let listener = TcpListener::bind(addr).unwrap();
    println!("listening on {}", port);

    loop {
        let (mut stream, _) = listener.accept().unwrap();
        std::thread::spawn(move || {
            echo_process(&mut stream).unwrap();
        });
    }
}
