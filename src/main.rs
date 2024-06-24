use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    collections::HashMap,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line=buf_reader.lines().map(|line|line.unwrap()).take_while(|line|!line.is_empty()).collect::<Vec<String>>();

    if request_line.is_empty(){
        send_404(&mut stream);
        return;
    }

    let parts:Vec<&str>=request_line[0].split_whitespace().collect();
    if parts.len()!=3{
        send_404(&mut stream);
        return;
    }
    let method=parts[0];
    let uri=parts[1];
    let _httpversion=parts[2];
    
    if method!="GET"{
        send_404(&mut stream);
        return;
    }

    if uri=="/"{
            stream.write_all("HTTP/1.1 200 OK\r\n\r\n".as_bytes()).unwrap();
    }else if uri=="/user-agent" {
        let headers:HashMap<String, String>=request_line[1..].iter()
            .filter_map(|x| x.split_once(':'))
            .map(|(key, value)| (key.trim().to_string(), value.trim().to_string()))
            .collect();
        if let  Some(user_agent) = headers.get("User-Agent"){
            let contents=user_agent;
            let length=contents.len();
            let status_line="HTTP/1.1 200 OK";
            let response=
                format!("{status_line}\r\nContent-Type:text/plain\r\nContent-Length:{length}\r\n\r\n{contents}");
            stream.write_all(response.as_bytes()).unwrap();
        }else{

            send_404(&mut stream);
            }
           
        
        }
    
    
}
fn send_404(stream: &mut TcpStream) {
    let status_line = "HTTP/1.1 404 Not Found";
    let response = format!("{status_line}\r\nContent-Type: text/plain\r\nContent-Length: 0\r\n\r\n");
    stream.write_all(response.as_bytes()).unwrap();
}