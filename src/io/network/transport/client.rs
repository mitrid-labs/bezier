use mitrid_core::base::Result;
use mitrid_core::base::Checkable;
use mitrid_core::io::network::ClientTransport as BasicClientTransport;

use std::net::{TcpStream, Shutdown};
use std::io::{Write, Read};
use std::mem;

pub const BUFFER_SIZE: usize = 2048;

use io::network::Address;

pub struct ClientTransport(pub TcpStream);

impl Clone for ClientTransport {
    fn clone(&self) -> ClientTransport {
        ClientTransport(self.0.try_clone().unwrap())
    }
}

impl Checkable for ClientTransport {}

impl BasicClientTransport<Address> for ClientTransport {
    fn connect(addresses: &Vec<Address>) -> Result<Self> {
        if addresses.len() != 1 {
            return Err(String::from("invalid length"));
        }

        let addr = addresses[0].to_owned();

        let tcp_stream = TcpStream::connect(&addr.to_string())
                            .map_err(|e| format!("{:?}", e))?;

        let ct = ClientTransport(tcp_stream);

        Ok(ct)
    }

    fn disconnect(&mut self) -> Result<()> {
        self.0.shutdown(Shutdown::Both)
            .map_err(|e| format!("{:?}", e)).into()
    }

    fn send(&mut self, data: &[u8]) -> Result<()> {
        if data.len() > BUFFER_SIZE -4 {
            return Err(format!("invalid length"));
        }

        let mut msg = [0u8; BUFFER_SIZE];

        let len_msg: [u8; 4] = unsafe { mem::transmute(data.len() as u32) };

        for i in 0..4 {
            msg[i] = len_msg[i];
        }

        for i in 0..data.len() {
            msg[i+4] = data[i];
        }

        self.0.write(&msg[..])
            .map(|_| ())
            .map_err(|e| format!("{:?}", e))
    }

    fn recv(&mut self) -> Result<Vec<Vec<u8>>> {
        let mut buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];

        self.0.read(&mut buffer[..])
            .map_err(|e| format!("{:?}", e))?;

        let mut len_msg = [0u8; 4];

        for i in 0..4 {
            len_msg[i] = buffer[i];
        }

        let len: u32 = unsafe { mem::transmute(len_msg) };

        let mut msg = Vec::new();

        for i in 0..len as usize {
            msg.push(buffer[i+4]);
        }

        Ok(vec![msg])
    }
}