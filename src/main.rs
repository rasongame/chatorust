use std::net::{TcpListener, TcpStream};
use std::thread;
use std::sync::{Arc, Mutex};
use std::io::prelude::*;
//use std::net::TcpStream;
//fn handler(stream: TcpStream) {
#[warn(unused_assignments)]

//}
fn main() -> std::io::Result<()> {
    let mut clients: Arc<Mutex<Vec<TcpStream>>> = Arc::new(Mutex::new(Vec::new()));
    let listener = TcpListener::bind("127.0.0.1:6666").unwrap();
    println!("listening started, ready to accept");
    for stream in listener.incoming() {
        let clients = Arc::clone(&clients);
        let _ = thread::spawn(move ||{
            let stream = stream.unwrap();
            let mut clone = clients.lock().unwrap();
            clone.push(stream);
            for mut client in clone.iter() {
                let mut strx = [0;128];
                println!(client.read(&mut strx));
                match client.write(&mut strx) {
                    Ok(_) => continue,
                    Err(_) => clone.push(*stream),
                }
            }
        });
    }
    Ok(())
}
