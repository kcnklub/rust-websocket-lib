

fn main() {

    let mut server = server::create_web_socket_server(8001); 

    server.start();

}