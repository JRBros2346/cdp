use crate::client::Request;
use std::net::{ToSocketAddrs, UdpSocket};

#[repr(C)]
#[derive(Debug)]
pub struct Response {
    pub device_id: u32,
    pub task_id: u32,
    pub result_format: u32,
    pub status: u32,
    pub result: Vec<u8>,
}

impl Response {
    pub fn serialize(&self) -> Vec<u8> {
        let mut data = Vec::with_capacity(16 + self.result.len());
        data.extend(&self.device_id.to_le_bytes());
        data.extend(&self.task_id.to_le_bytes());
        data.extend(&self.result_format.to_le_bytes());
        data.extend(&self.status.to_le_bytes());
        data.extend(&self.result);
        data
    }
    pub fn deserialize(data: &[u8]) -> Self {
        let device_id = u32::from_be_bytes(data[0..4].try_into().unwrap());
        let task_id = u32::from_be_bytes(data[4..8].try_into().unwrap());
        let result_format = u32::from_be_bytes(data[8..12].try_into().unwrap());
        let status = u32::from_be_bytes(data[12..16].try_into().unwrap());
        let result = data[16..].to_vec();
        Self {
            device_id,
            task_id,
            result_format,
            status,
            result,
        }
    }
}

pub struct CdpServer(UdpSocket);

impl CdpServer {
    pub fn bind<A: ToSocketAddrs>(addr: A) -> std::io::Result<Self> {
        UdpSocket::bind(addr).map(Self)
    }
    pub fn run(&self) -> std::io::Result<()> {
        let mut buf = [0u8; 65536];
        loop {
            let (size, sender) = self.0.recv_from(&mut buf)?;
            let request = Request::deserialize(&buf[..size]);
            println!("{} {}", request.device_id, request.task_id);
            let response = Response {
                device_id: request.device_id,
                task_id: request.task_id,
                result_format: request.result_format,
                status: 0,
                result: vec![],
            };
            let response = response.serialize();
            self.0.send_to(&response, sender)?;
        }
    }
}
