use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
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

    let request_line=buf_reader.lines().next().unwrap().unwrap();
    //Resolusion 1  :按行解析，把status_line作为一个整体
    // let (status_line,filename)=if request_line=="GET / HTTP/1.1"{
    //     ("HTTP/1.1 200 OK","hello.html")
    // }else{
    //     ("HTTP/1.1 404 Not Found","404.html")
    // };

    //Resolusion 2  :按词解析，因为status_line是规则的，method+space+uri+space+httpversion+CR+LF
    let parts:Vec<&str>=request_line.split_whitespace().collect();
    if parts.len()==3{
        let method=parts[0];
        let uri=parts[1];
        let httpversion=parts[2];
    }

    let parts2:Vec<&str>=uri.split("/").collect();
    if parts2.len()==2{
        let contents=parts2[1];
        if parts2[0]=='echo' {
            status_line="HTTP/1.1 200 OK";
            length=contents.len();
            let response=
        // format!("{status_line}\r\nContent-Length:{length}\r\n\r\n{contents}");
        // format!("{status_line}\r\n\r\n{contents}");
                format!("{status_line}\r\nContent-Type:text/plain\r\nContent-Length:{length}\r\n\r\n{contents}");
        }
    }else{
        status_line="HTTP/1.1 404 Not Found";
        let response=format!("{status_line}\r\n\r\n");
    }


    // let contents=fs::read_to_string(filename).unwrap();
    // let length=contents.len();
    stream.write_all(response.as_bytes()).unwrap();    
       
}