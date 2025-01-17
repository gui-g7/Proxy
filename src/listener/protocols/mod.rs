use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::ipv4::Ipv4Packet;
use std::sync::Arc;
use crate::listener::metrics::TrafficMetrics;

pub mod tcp;
pub mod udp;
pub mod icmp;

/// Função que recebe um pacote IPv4 e identifica o protocolo para redirecionamento.
pub fn process_ip_packet(packet: &Ipv4Packet, metrics: &Arc<TrafficMetrics>) {
    match packet.get_next_level_protocol() {
        IpNextHeaderProtocols::Tcp => {
            tcp::process_tcp_packet(packet, metrics);
        }
        IpNextHeaderProtocols::Udp => {
            udp::process_udp_packet(packet, metrics);
        }
        IpNextHeaderProtocols::Icmp => {
            icmp::process_icmp_packet(packet, metrics);
        }
        _ => {
            println!("Protocolo não suportado: {:?}", packet.get_next_level_protocol());
        }
    }
}
