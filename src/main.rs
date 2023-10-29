use std::io::Write;

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, TcpListener, TcpStream};
use chrono::Utc;
use threadpool::ThreadPool;

fn main() {
    let worker_count = 4;
    let pool = ThreadPool::new(worker_count);
    let tcp_port = 11111;
    let socket_v4_tcp = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), tcp_port);
    let socket_v6_tcp = SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0)), tcp_port);

    let http_port = 8080;
    let socket_v4_http = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), http_port);
    let socket_v6_http = SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0)), http_port);

    let socket_addrs = vec![socket_v4_tcp, socket_v6_tcp, socket_v4_http, socket_v6_http];
    let listener = TcpListener::bind(&socket_addrs[..]);
    if let Ok(listener) = listener {
        println!("Listening on {}:{}", listener.local_addr().unwrap().ip(), listener.local_addr().unwrap().port());
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            let addr =stream.peer_addr().unwrap().ip().to_string();
            if stream.local_addr().unwrap_or(socket_v4_http).port() == tcp_port {
                pool.execute(move||send_tcp_response(stream, addr));
            } else {
                //http might be proxied via https so let anything which is not the tcp port be http
                pool.execute(move||send_http_response(stream, addr));
            }
        }
    } else {
        println!("Unable to bind to port")
    }
}

fn send_tcp_response(mut stream:TcpStream, addr:String) {
    stream.write_all(addr.as_bytes()).unwrap();
}

fn send_http_response(mut stream:TcpStream, addr:String) {

    let html = format!("<html><head><title>{}</title></head><body><h1>{}</h1></body></html>", addr, addr);
    let length = html.len();
    let response = format!("HTTP/1.1 200 OK\r\nContent-Length: {length}\r\n\r\n{html}" );
    stream.write_all(response.as_bytes()).unwrap();
    println!("{}\tHTTP\t{}",Utc::now().to_rfc2822(),addr)
}