use std::{thread, io::{Read, Write}, net::{TcpListener, TcpStream}};

// 处理客户端请求
fn handle_client(mut stream: TcpStream) {

    // 读取请求stream 内容
    let mut buffer = [0;512];
    stream.read(&mut buffer).unwrap();

    // 打印请求消息
    println!("接收到一个请求------\n");
    println!("{}", String::from_utf8_lossy(&buffer[..]));
    println!("\n------请求内容结束.\n");



    // 创建头部模板方法
    fn head_template(status:String) -> String {
        format!("HTTP/1.1 {}\r\n\r\n", status)
    }
    
    // 解构 构建响应头 和 响应消息
    let (head, content) = (head_template(String::from("200 OK")), String::from("我已经收到您发过来的消息了。"));

    // let get = b"GET /";
    // let is_get = buffer.starts_with(get);
    // match is_get {
    //     true => (head_template(String::from("200 OK")), String::from("我已经收到您发过来的消息了。")),
    //     false => (head_template("404 NO FOUND 抱歉暂时不支持get以外的请求".to_string()), String::from("404"))
    // };

    // 格式化组装 response 响应消息体
    let response = format!("{}{}", head, content);
    // 把响应内容写进stream
    stream.write(&response.as_bytes()).unwrap();
    // 响应消息
    stream.flush().unwrap();
}

fn main() -> std::io::Result<()> {
    // 启动一个3030端口的本地服务
    let listener = TcpListener::bind("127.0.0.1:3030").unwrap();
    // 创建一个thread_vec
    let mut thread_vec: Vec<thread::JoinHandle<()>> = Vec::new();

    // 按顺序处理 客户端发过来的请求
    for stream in listener.incoming() {
        let stream_result = stream.unwrap();

        // 创建线程处理请求
        let handle = thread::spawn(move || {
            handle_client(stream_result);
        });
        // 把请求处理 推到 管道里面
        thread_vec.push(handle);
    }

    // 处理请求的线程
    for handle in thread_vec {
        handle.join().unwrap();
    }

    // 异常处理成功 返回 Ok(())
    Ok(())
}