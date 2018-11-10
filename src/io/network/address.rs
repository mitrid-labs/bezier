use mitrid_core::base::{Sizable, VariableSize};
use mitrid_core::base::Checkable;
use mitrid_core::base::Serializable;
use mitrid_core::base::Datable;

use std::net::SocketAddr;
use std::str::FromStr;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, Serialize, Deserialize)]
pub struct Address(pub SocketAddr);

impl Address {
    pub fn new(sock: SocketAddr) -> Address {
        Address(sock)
    }

    pub fn to_string(&self) -> String {
        format!("{}", self.0)
    }
}

pub const DEFAULT_SOCKET: &str = "127.0.0.1:6173";

impl Default for Address {
    fn default() -> Address {
        let socket = SocketAddr::from_str(DEFAULT_SOCKET).unwrap();
        Address::new(socket)
    }
}

impl Sizable for Address {
    fn size(&self) -> u64 {
        32
    }
}

impl VariableSize for Address {
    fn min_size() -> u64 {
        32
    }

    fn max_size() -> Option<u64> {
        Some(32)
    }
}

impl Checkable for Address {}

impl Serializable for Address {}

impl Datable for Address {}