#![warn(
    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
)]

use anyhow::{Result};
use std::env;
use std::io::{BufReader, BufRead, self};
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncWriteExt, AsyncBufReadExt, BufReader as AsyncBufReader};

#[tokio::main]
async fn main() -> Result<()> {
    let mut args = env::args();
    args.next();
    if let Some(arg) = args.next() {
        if arg == "client" {
            client().await?;
        } else if arg == "server" {
            server().await?;
        }
    }
    Ok(())
}


async fn client() -> Result<()> {
    let stdin =io::stdin(); 
    let mut buffer = String::new();
    let socket = TcpStream::connect("127.0.0.1:5454").await?;
    let (rd, mut wt) = socket.into_split();
    let mut socket_reader = AsyncBufReader::new(rd);
    loop {
        buffer.clear();
        let lock = stdin.lock();
        let mut reader = BufReader::new(lock);
        reader.read_line(&mut buffer)?;
        wt.write_all(buffer.as_bytes()).await?;
        socket_reader.read_line(&mut buffer).await?;
        println!("{}", buffer);
    }
}



async fn server() -> Result<()> {
    let mut listener = TcpListener::bind("127.0.0.1:5454").await?;
    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(server_handler(socket));
    }
}

async fn server_handler(socket: TcpStream) -> Result<()> {
    let mut str = String::new();
    let (rd, mut wt) = socket.into_split();
    let mut reader = AsyncBufReader::new(rd);
    loop {
        str.clear();
        reader.read_line(&mut str).await?;
        str.truncate(str.len() - 1);
        dbg!(&str);
        str.insert_str(str.len(), &str.chars().rev().collect::<String>());
        str.insert(str.len(), '\n');
        dbg!(&str);
        wt.write_all(str.as_bytes()).await?;
    }
}