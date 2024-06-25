use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    collections::HashMap,
    thread,
    env,
};

fn main() {
    let args:Vec<String>=env::args().iter().collect();
    let directory= if let Some(dir)=args.iter().position(|| x=="--directory"){
        &args[dir+1]
    }
    else{
        eprintln!("Usage:{}",args[0]);
    }

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let directory=directory.to_string();    
        handle_connection(stream,&directory);
        
        
    }
}

fn handle_connection(mut stream: TcpStream,directory:&str) {
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
// if uri=="files"{} statrt with  剩余部分为文件名， 文件名是否存在，contents=文件名。read_to_string 
    if uri=="/"{
            stream.write_all("HTTP/1.1 200 OK\r\n\r\n".as_bytes()).unwrap();
    }else if uri.starts_with("/echo/"){
        let contents=&uri[6..];
        let length=contents.len();
        let status_line="HTTP/1.1 200 OK";
        let response=
            format!("{status_line}\r\nContent-Type:text/plain\r\nContent-Length:{length}\r\n\r\n{contents}");
        stream.write_all(response.as_bytes()).unwrap();
    }else if uri.starts_with("/files/"){
        let filename=&uri[7..];
        let mut filepath=Pathbuf::from(directory);
        filepath.push(filename);

        match fs::read(&filepath){
            Ok(contents)=>{
                let length=contents.len();
                let status_line="HTTP/1.1 200 OK";
                let response=
                    format!("{status_line}\r\nContent-Type:application/octet-stream\r\nContent-Length:{length}\r\n\r\n{contents}");
                stream.write_all(response.as_bytes()).unwrap();
            }
            Err(_)=>{
                send_404(&mut stream);
            }
        }
        // let length=contents.len();
        // let status_line="HTTP/1.1 200 OK";
        // let response=
        //     format!("{status_line}\r\nContent-Type:text/plain\r\nContent-Length:{length}\r\n\r\n{contents}");
        // stream.write_all(response.as_bytes()).unwrap();
    else if uri=="/user-agent" {
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
        else {
            send_404(&mut stream);
            return;        
        }

    
    
}
fn send_404(stream: &mut TcpStream) {
    let status_line = "HTTP/1.1 404 Not Found";
    let response = format!("{status_line}\r\nContent-Type: text/plain\r\nContent-Length: 0\r\n\r\n");
    stream.write_all(response.as_bytes()).unwrap();
}