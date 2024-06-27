use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    collections::HashMap,
    thread,
    env,
    path::PathBuf,
    sync::Arc,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        thread::spawn(move || {
            handle_connection(stream);
        });  
}   
}
        
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    // let request_line=buf_reader.lines().map(|line|line.unwrap()).take_while(|line|!line.is_empty()).collect::<Vec<String>>();
    let request_line=buf_reader.lines().map(|line|line.unwrap()).collect::<Vec<String>>();
      
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
    

    if uri=="/"{
            stream.write_all("HTTP/1.1 200 OK\r\n\r\n".as_bytes()).unwrap();
    }
    else if uri.starts_with("/echo/"){
        let contents=&uri[6..];
        let length=contents.len();
        let status_line="HTTP/1.1 200 OK";
        let response=
            format!("{status_line}\r\nContent-Type:text/plain\r\nContent-Length:{length}\r\n\r\n{contents}");
        stream.write_all(response.as_bytes()).unwrap();
    }
    else if uri.starts_with("/files/"){
        let args:Vec<String>=env::args().collect();
        let dir_index= args.iter()
            .position(|x| x=="--directory")
            .expect("no path ");
        let directory=&args[dir_index+1];
        let filename=&uri[7..];
        let mut filepath=PathBuf::from(&directory);
        filepath.push(filename);

        if method=="GET" {
            match fs::read(&filepath){
                Ok(contents)=>{
                    let length=contents.len();
                    let status_line="HTTP/1.1 200 OK";
                    let response=
                        format!("{status_line}\r\nContent-Type:application/octet-stream\r\nContent-Length:{length}\r\n\r\n");
                    stream.write_all(response.as_bytes()).unwrap();
                    stream.write_all(&contents).unwrap();
                }
                Err(_)=>{
                    send_404(&mut stream);
                }
            }
        }
        else if method=="POST"{
            let empty_line_index: Vec<usize> = request_line.iter()
                .enumerate()
                .filter_map(|(index, line)| if line.is_empty() { Some(index) } else { None })
                .collect();


            // let empty_line_index=request_line.iter().position(|x| x.is_empty()).unwrap();
            let content=request_line[empty_line_index[0]+1].clone();
            let length=content.len();
            match fs::File::create(&filepath){
                Ok(mut file)=>{
                    match file.write_all(content.as_bytes()){
                        Ok(_)=>{
                            
                            let response=format!("HTTP/1.1 201 Created\r\n\r\n");
                            stream.write_all(response.as_bytes()).unwrap();
                        
                        },
                        Err(e)=>{
                            send_404(&mut stream);
                            return;
                        }
                    }
                },
                Err(e)=>{
                    let status_line="HTTP/1.1 500 Internal Server Error";
                    let response=format!("{status_line}\r\n\r\n");
                    stream.write_all(response.as_bytes()).unwrap();
                    return;
                }
            };
        }
        else{
            send_404(&mut stream);
        }
    } 
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
        
    }else {
            send_404(&mut stream);
            return;        
    }

   
}
fn send_404(stream: &mut TcpStream) {
    let status_line = "HTTP/1.1 404 Not Found";
    let response = format!("{status_line}\r\nContent-Type: text/plain\r\nContent-Length: 0\r\n\r\n");
    stream.write_all(response.as_bytes()).unwrap();
}