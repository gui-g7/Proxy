use pnet::packet::udp::UdpPacket;
use pnet::packet::Packet;
use crate::listener::metrics::TrafficMetrics;
use std::sync::Arc;

pub fn process_raw_udp_packet(packet: &[u8], metrics: &Arc<TrafficMetrics>) {
    if let Some(udp_packet) = UdpPacket::new(packet) {
        let src_port = udp_packet.get_source();
        let dest_port = udp_packet.get_destination();
        let payload_size = packet.len() as u64;

        println!(
            "Raw UDP Packet: Src Port: {}, Dest Port: {}, Size: {} bytes",
            src_port, dest_port, payload_size
        );

        // Atualiza métricas
        metrics.add_data("Raw UDP", payload_size);
    } else {
        println!("Failed to parse UDP packet.");
    }
}
