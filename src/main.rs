use std::net::{TcpListener, TcpStream};
use std::io::{
    Write,
    //Read
};

fn handle_client(stream: TcpStream, clients: &mut Vec<TcpStream>) {
    println!("{:?}", stream);
    clients.push(stream);
    for client in clients {
        client.write("Hello".as_bytes()).unwrap();
    }
}

fn main() -> std::io::Result<()> {
    let mut clients = vec![];
    let listener = TcpListener::bind("127.0.0.1:6666")?;
    for connection in listener.incoming() {
        handle_client(connection?.try_clone()?, &mut clients);
    }
    Ok(())
}
