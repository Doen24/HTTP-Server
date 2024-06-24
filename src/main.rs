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
    let parts:Vec<&str>=request_line[0].split_whitespace().collect();
    let method=parts[0];
    let uri=parts[1];
    let httpversion=parts[2];
    if method=="GET"{
        if uri=="/"{
            stream.write_all("HTTP/1.1 200 OK\r\n\r\n".as_bytes()).unwrap();
        }else if uri.starts_with("/user-agent"){
            let headers:HashMap<&str,&str>=request_line[1].lines().map(|x| x.split_once(':').expect("Invaild header format")).collect();
            let key="User-Agent";
            match headers.get(key){
                Some(value)=>{
                    let contents=value;
                    let length=contents.len();
                    let status_line="HTTP/1.1 200 OK";
                    let response=
                        format!("{status_line}\r\nContent-Type:text/plain\r\nContent-Length:{length}\r\n\r\n{contents}");
                        stream.write_all(response.as_bytes()).unwrap();
                }
                None=>{
                    stream.write_all("HTTP/1.1 404 Not Found\r\n\r\n".as_bytes()).unwrap();
                }
            }
           
        
        }else{
            stream.write_all("HTTP/1.1 404 Not Found\r\n\r\n".as_bytes()).unwrap();
        }
    }else{
        stream.write_all("HTTP/1.1 404 Not Found\r\n\r\n".as_bytes()).unwrap();
    }
}
   
    // let request_line=buf_reader.lines().next().unwrap().unwrap();
    // // Resolusion 1  :按行解析，把status_line作为一个整体
    // let (status_line,filename)=if request_line=="GET /echo/{} HTTP/1.1"{
    //     ("HTTP/1.1 200 OK","hello.html")
    // }else{
    //     ("HTTP/1.1 404 Not Found","404.html")
    // };
    // //Resolusion 2  :按词解析，因为status_line是规则的，method+space+uri+space+httpversion+CR+LF
   
      