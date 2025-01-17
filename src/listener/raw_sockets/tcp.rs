use pnet::packet::tcp::TcpPacket;
use crate::listener::metrics::TrafficMetrics;
use std::sync::Arc;

#[allow(unused)]
pub fn process_raw_tcp_packet(packet: &[u8], metrics: &Arc<TrafficMetrics>) {
    if let Some(tcp_packet) = TcpPacket::new(packet) {
        let src_port = tcp_packet.get_source();
        let dest_port = tcp_packet.get_destination();
        let payload_size = packet.len() as u64;

        println!(
            "Raw TCP Packet: Src Port: {}, Dest Port: {}, Size: {} bytes",
            src_port, dest_port, payload_size
        );

        // Atualiza métricas
        metrics.add_data("Raw TCP", payload_size);
    } else {
        println!("Failed to parse TCP packet.");
    }
}
