use std::net::{ToSocketAddrs, UdpSocket};

#[repr(C)]
#[derive(Debug)]
pub struct Request {
    pub device_id: u32,
    pub task_id: u32,
    pub result_format: u32,
    pub bytecode_format: u32,
    pub bytecode: Vec<u8>,
}

impl Request {
    pub fn serialize(&self) -> Vec<u8> {
        let mut data = Vec::with_capacity(16 + self.bytecode.len());
        data.extend(&self.device_id.to_le_bytes());
        data.extend(&self.task_id.to_le_bytes());
        data.extend(&self.result_format.to_le_bytes());
        data.extend(&self.bytecode_format.to_le_bytes());
        data.extend(&self.bytecode);
        data
    }
    pub fn deserialize(data: &[u8]) -> Self {
        let device_id = u32::from_be_bytes(data[0..4].try_into().unwrap());
        let task_id = u32::from_be_bytes(data[4..8].try_into().unwrap());
        let result_format = u32::from_be_bytes(data[8..12].try_into().unwrap());
        let bytecode_format = u32::from_be_bytes(data[12..16].try_into().unwrap());
        let bytecode = data[16..].to_vec();
        Self {
            device_id,
            task_id,
            result_format,
            bytecode_format,
            bytecode,
        }
    }
}

pub struct CdpClient(UdpSocket);
impl CdpClient {
    pub fn bind<A: ToSocketAddrs>(addr: A) -> std::io::Result<Self> {
        let socket = UdpSocket::bind("127.0.0.1:0").map(Self)?;
        socket.0.connect(addr)?;
        Ok(socket)
    }
    pub fn send(&self, request: Request) -> std::io::Result<()> {
        let data = request.serialize();
        self.0.send(&data)?;
        Ok(())
    }
}
