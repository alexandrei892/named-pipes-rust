use tokio::io::Interest;
use tokio::net::windows::named_pipe;
use std::error::Error;
use std::io;
use std::str;


const PIPE_NAME: &str = r"\\.\pipe\testpipe";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = named_pipe::ClientOptions::new().open(PIPE_NAME)?;

    loop {
        let ready = client.ready(Interest::READABLE | Interest::WRITABLE).await?;

        if ready.is_readable() {
            let mut data = vec![0; 4096];
            // Try to read data, this may still fail with `WouldBlock`
            // if the readiness event is a false positive.
            match client.try_read(&mut data) {
                Ok(n) => {
                    println!("read {} bytes", n);
                    
                    let s = match str::from_utf8(&data) {
                        Ok(v) => v,
                        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                    };
                
                    println!("result: {}", s);
                }
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                    continue;
                }
                Err(e) => {
                    return Err(e.into());
                }
            }
        }

        if ready.is_writable() {
            // Try to write data, this may still fail with `WouldBlock`
            // if the readiness event is a false positive.
            match client.try_write(b"hello world\n") {
                Ok(n) => {
                    println!("write {} bytes", n);
                }
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                    continue;
                }
                Err(e) => {
                    return Err(e.into());
                }
            }
        }
    }
}