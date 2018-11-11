use mitrid_core::base::Result;
use mitrid_core::base::Checkable;
use mitrid_core::io::network::ClientTransport as BasicClientTransport;
use mitrid_core::io::network::ServerTransport as BasicServerTransport;

use std::net::TcpListener;

pub const BUFFER_SIZE: usize = 2048;

use io::network::Address;
use io::network::transport::ClientTransport;

pub struct ServerTransport(pub TcpListener);

impl ServerTransport {
    pub fn serve_ping(address: &Address) {
        let mut server = ServerTransport::listen(address).unwrap();

        let (mut client, _) = server.accept().unwrap();
        
        let msg = client.recv().unwrap();

        client.send(msg.as_slice()).unwrap();
    }
}

impl Checkable for ServerTransport {}

impl BasicServerTransport<Address, ClientTransport> for ServerTransport {
    fn listen(address: &Address) -> Result<ServerTransport> {
        let listener = TcpListener::bind(&address.to_string())
                            .map_err(|e| format!("{:?}", e))?;

        let st = ServerTransport(listener);

        Ok(st)
    }

    fn accept(&mut self) -> Result<(ClientTransport, Address)> {
        let (tcp_stream, socket) = self.0.accept()
                                        .map_err(|e| format!("{:?}", e))?;

        let transport = ClientTransport(tcp_stream);
        let address = Address::from_socket(&socket);

        Ok((transport, address))
    }

    fn close(&mut self) -> Result<()> {
        Ok(())
    }
}