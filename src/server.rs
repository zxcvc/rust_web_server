use std::{
    net,
    fs,
    fs::File,
    io,
    io::{Read, Write},
};

pub fn file_reader(file_name: &str) -> Option<String> {
    let file = File::open(file_name);
    let mut buffer = String::new();
    match file {
        Result::Ok(mut file) => match file.read_to_string(&mut buffer) {
            Result::Ok(_) => Option::Some(buffer),
            Result::Err(_) => Option::None,
        },
        Result::Err(_) => Option::None,
    }
}

pub fn handler(stream: &mut net::TcpStream) {
    let ret = file_reader("./sources/index.html");
    match ret {
        Option::Some(content) => {
            let mut buffer = [0; 1024];
            stream.read(&mut buffer).expect("缓冲区读取失败");
            stream.write("HTTP/1.1 200 OK\r\n\r\n".as_bytes()).expect("缓冲区写入失败");
            stream.write(content.as_bytes()).expect("缓冲区写入失败");
            stream.flush().expect("缓冲区刷新失败");
        }
        Option::None => {}
    }
}

pub fn run(port: &str) {
    let host = "localhost:";
    let listener = net::TcpListener::bind(host.to_string() + port).expect("监听端口失败");
    for item in listener.incoming() {
        match item {
            Result::Ok(mut stream) => {
                handler(&mut stream);
            }
            Result::Err(err) => {
                println!("{}", err);
            }
        }
    }
}
