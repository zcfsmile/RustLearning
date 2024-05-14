use std::io;
use std::net::TcpListener;
use std::thread::spawn;

fn main() {
    echo_main("127.0.0.1:17007").expect("error: ");
}

fn echo_main(addr: &str) -> io::Result<()> {
    let listener = TcpListener::bind(addr)?;
    println!("listening on {}", addr);
    loop {
        // 等待客户端进入
        let (mut stream, addr) = listener.accept()?;
        println!("connection received from {}", addr);

        // 启动一个线程来处理此客户端
        let mut write_steam = stream.try_clone()?;
        spawn(move || {
            // 回显从 stream 收到的一切
            io::copy(&mut stream, &mut write_steam).expect("error in client thread: ");
            println!("connection closed");
        });
    }
}
