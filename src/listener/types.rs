use pnet::packet::ip::IpNextHeaderProtocol;
use std::net::SocketAddrV4;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ConnectionIdentifier {
    pub protocol: IpNextHeaderProtocol,
    pub source: SocketAddrV4,
    pub destination: SocketAddrV4,
}

#[allow(unused)]
#[derive(Debug, Clone, PartialEq)]
pub enum ConnectionState {
    New,
    Established,
    Closing,
    Closed,
}

#[allow(unused)]
#[derive(Debug, Clone, PartialEq)]
pub struct PacketInfo {
    pub src_ip: String,
    pub dst_ip: String,
    pub src_port: u16,
    pub dst_port: u16,
    pub payload_size: u64,
}

#[allow(unused)]
pub trait ProcessPacket {
    fn process(&self, packet: &PacketInfo);
}
#[allow(unused)]
#[derive(Debug, Clone)]
pub struct DnsRecord {
    pub ip: String,
    pub service: String,
}

#[allow(unused)]
impl DnsRecord {
    pub fn new(ip: &str, service: &str) -> Self {
        Self {
            ip: ip.to_string(),
            service: service.to_string(),
        }
    }
}
