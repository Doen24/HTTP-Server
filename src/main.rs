use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    collections::HashMap,
    thread,
    env,
    path::PathBuf,
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
#[derive(Debug)]
struct Http_request{
    method:String,
    uri:String,
    httpversion:String,
    headers:HashMap<String,String>,
    body:String,
}
impl Http_request{
    //收到request，分解成method,uri,httpversion,headers,body
    fn parsing(stream:&TcpStream)->Http_request{
        let mut buf_reader = BufReader::new(stream);
        let mut request_line=String::new();
        buf_reader.read_line(&mut request_line).unwrap();
        //find empty str in request_line
        //空行转为空字符串"",而不是空格字符串" "
        //let empty_index=request_line.iter().position(|x| x=="").unwrap();
        let parts:Vec<&str>=request_line.trim_end().split_whitespace().collect();
        let method=parts[0];
        let uri=parts[1];
        let httpversion=parts[2];

        let mut headers=HashMap::new();
        loop{
            let mut line=String::new();
            buf_reader.read_line(&mut line).unwrap();
            let line=line.trim_end();
            if line.is_empty(){
                break;
            }
            if let Some((key, value)) = line.split_once(':') {
                headers.insert(key.trim().to_string(), value.trim().to_string());
            }
        }

        let mut body = String::new();
        if let Some(content_length) = headers.get("Content-Length") {
            let content_length: usize = content_length.parse().unwrap();
            let mut buffer = vec![0; content_length];
            buf_reader.read_exact(&mut buffer).unwrap();
            body = String::from_utf8(buffer).unwrap();
        }
       
        Http_request{
            method:method.to_string(),
            uri:uri.to_string(),
            httpversion:httpversion.to_string(),
            headers:headers,
            body:body,
        }
    }
}

fn config()->String{
    let args:Vec<String>=env::args().collect();
    let dir_index= args.iter()
        .position(|x| x=="--directory");
    if let Some(index) = dir_index {
            // 确保在 --directory 之后有一个参数
        if index + 1 < args.len() {
            let directory = args[index + 1].clone();
            return directory;
        } else {
            eprintln!("Error: No path provided after --directory");
            std::process::exit(1);
        }
    } else {
        eprintln!("Error: --directory parameter not found");
        std::process::exit(1);
    }
}
fn handle_connection(mut stream: TcpStream) {
    let http_request=Http_request::parsing(&stream);
    match http_request.method.as_str(){
        //GET uppercase 大写
        "GET"=>{
            if http_request.uri=="/"{
                stream.write_all("HTTP/1.1 200 OK\r\n\r\n".as_bytes()).unwrap();
            }
            else if http_request.uri.starts_with("/echo/"){
                let contents=&http_request.uri[6..];
                let length=contents.len();
                let status_line="HTTP/1.1 200 OK";
                if let Some(compression)=http_request.headers.get("Accept-Encoding"){
                    let encodings=compression.split(",").map(|x| x.trim()).collect::<Vec<&str>>();
                    if encodings.contains(&"gzip"){
                        let response=
                            format!("{status_line}\r\nContent-Type: text/plain\r\nContent-Encoding: gzip\r\n\r\n");
                        stream.write_all(response.as_bytes()).unwrap();
                    }else{
                        let response=
                            format!("{status_line}\r\nContent-Type:text/plain\r\n\r\n");
                        stream.write_all(response.as_bytes()).unwrap();
                    }
                }else{
                    let response=
                        format!("{status_line}\r\nContent-Type:text/plain\r\nContent-Length:{length}\r\n\r\n{contents}");
                    stream.write_all(response.as_bytes()).unwrap();
                }
                
            }
            else if http_request.uri.starts_with("/files/"){
                let directory=config();
                let filename=&http_request.uri[7..];
                let mut filepath=PathBuf::from(&directory);
                filepath.push(filename);
                // 调试输出
                println!("Trying to read file: {:?}", filepath);
                match fs::read(&filepath){
                    Ok(contents)=>{
                        let length=contents.len();
                        let status_line="HTTP/1.1 200 OK";
                        let response=
                            format!("{status_line}\r\nContent-Type:application/octet-stream\r\nContent-Length:{length}\r\n\r\n");
                        stream.write_all(response.as_bytes()).unwrap();
                        stream.write_all(&contents).unwrap();
                    }
                    Err(err)=>{
                        eprintln!("Error reading file {:?}:{:?}",filepath,err);
                        send_404(&mut stream);
                    }
                }
            } 
            else if http_request.uri=="/user-agent" {
                let headers:HashMap<String, String>=http_request.headers;
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
        "POST"=>{
            if http_request.uri.starts_with("/files/"){
                let directory=config();
                let filename=&http_request.uri[7..];
                let mut filepath=PathBuf::from(directory);
                filepath.push(filename);
                match fs::File::create(&filepath){
                    Ok(mut file)=>{
                        match file.write(http_request.body.as_bytes()){
                            Ok(_)=>{
                                let response=format!("HTTP/1.1 201 Created\r\n\r\n");
                                stream.write_all(response.as_bytes()).unwrap();
                            },
                            Err(_)=>{
                                send_404(&mut stream);
                                return;
                            }
                        }
                    },
                    Err(_)=>{
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
        _=>{
            send_404(&mut stream);
        }
   }
      
}

fn send_404(stream: &mut TcpStream) {
    let status_line = "HTTP/1.1 404 Not Found";
    let response = format!("{status_line}\r\n\r\n");
    stream.write_all(response.as_bytes()).unwrap();
}