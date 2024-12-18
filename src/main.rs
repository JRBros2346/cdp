use cdp::client::{CdpClient, Request};

fn main() -> std::io::Result<()> {
    let thread = std::thread::spawn(|| -> std::io::Result<()> {
        use cdp::server::CdpServer;
        CdpServer::bind("127.0.0.1:8000")?.run()
    });
    let client = CdpClient::bind("127.0.0.1:8000")?;
    client.send(Request {
        device_id: 0,
        task_id: 0,
        result_format: 0,
        bytecode_format: 0,
        bytecode: vec![],
    })?;
    thread.join().expect("CAN'T JOIN")?;
    Ok(())
}
