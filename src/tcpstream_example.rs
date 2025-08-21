use std::net::{TcpStream, Shutdown};
use std::io::{Read, Write};

fn main() {
    // 连接到本地的 8080 端口
    match TcpStream::connect("127.0.0.1:8080") {
        Ok(mut stream) => {
            println!("已连接到服务器");

            // 发送数据到服务器
            let msg = b"Hello, server!";
            stream.write(msg).unwrap();

            // 从服务器读取响应
            let mut buffer = [0; 512];
            match stream.read(&mut buffer) {
                Ok(size) => {
                    println!("收到响应: {}", String::from_utf8_lossy(&buffer[..size]));
                },
                Err(e) => {
                    println!("读取失败: {}", e);
                }
            }

            // 关闭连接
            stream.shutdown(Shutdown::Both).unwrap();
        },
        Err(e) => {
            println!("连接失败: {}", e);
        }
    }
}