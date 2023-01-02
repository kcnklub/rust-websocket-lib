use std::{
    io::{Read, Write},
    net::TcpStream,
    str
};

fn main()
{
    println!("Hello, world!");

    let mut connection = TcpStream::connect("localhost:8001").unwrap();

    // Will have to worry about authority processing when working on building the client out correctly.
    let mut client_handshake = String::from("GET / HTTP/1.1\r\n"); // the first line will always be a GET method to create the connection.
    client_handshake.push_str("Upgrade: websocket\r\n");
    client_handshake.push_str("Connection: Upgrade\r\n");
    client_handshake.push_str("Sec-WebSocket-Key: keytest\r\n");
    match connection.write(client_handshake.as_bytes())
    {
        Ok(size) => println!("Sent handshake successfully, {} bytes", size),
        Err(error) => eprintln!("Error sending handshake data, {error}"),
    }

    let mut buffer = [0; 1024];
    match connection.read(&mut buffer)
    {
        Ok(size) => println!("Read handshake from server successfully, {} bytes", size),
        Err(error) => eprintln!("Error reading handshake data from server, {error}"),
    }

    let message = str::from_utf8(&buffer).unwrap();

    // need to validate the websocket accept cookie on thie response.



    println!("{message}");
}
