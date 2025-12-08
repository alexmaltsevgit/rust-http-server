use crate::http::request::Request;
use crate::http::response::Response;
use crate::http::router::{Router, RouterPath};
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};

mod http;

fn main() {
    let tries: [usize; 5] = core::array::from_fn(|i| i + 1);
    let addrs: Vec<SocketAddr> = tries
        .iter()
        .map(|&x| SocketAddr::from(([127, 0, 0, 1], 8110 + x as u16)))
        .collect();

    let listener = TcpListener::bind(&addrs[..]).unwrap();

    println!("Bind at {}", listener.local_addr().unwrap());

    let mut router = Router::new();
    router.get(
        "/",
        &[&|req, res| {
            println!("Принято ебать");
        }],
    );

    listener.incoming().for_each(|x| {
        if let Ok(stream) = x {
            handle_stream(&mut router, stream);
        }
    });
}

fn handle_stream(router: &mut Router, mut stream: TcpStream) -> Result<(), ()> {
    let buf_reader = BufReader::new(&stream);

    let mut lines = buf_reader.lines();

    let request_status_line = lines.next().unwrap().unwrap();
    let (router_path, _) = request_status_line.trim().rsplit_once(" ").unwrap();

    let router_path: RouterPath = router_path.parse()?;

    let headers: HashMap<_, _> = HashMap::from_iter(
        lines
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .map(|header_string| {
                let strs = header_string.split_once(": ").unwrap();
                (strs.0.to_string(), strs.1.to_string())
            }),
    );

    let req = Request::new(headers);
    let mut res = Response::new();

    router.run_route_middleware_chain(&router_path, &req, &mut res);

    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write_all(response.as_bytes()).unwrap();

    Ok(())
}
