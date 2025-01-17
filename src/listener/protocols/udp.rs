use crate::listener::metrics::TrafficMetrics;
use pnet::packet::udp::UdpPacket;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::Packet;
use std::sync::Arc;

pub fn process_udp_packet(ipv4_packet: &Ipv4Packet, metrics: &Arc<TrafficMetrics>) {
    if let Some(udp_packet) = UdpPacket::new(ipv4_packet.payload()) {
        let src_port = udp_packet.get_source();
        let dst_port = udp_packet.get_destination();
        let payload_size = ipv4_packet.payload().len() as u64;

        println!("=== Pacote UDP ===");
        println!("Endereço IP de Origem: {}", ipv4_packet.get_source());
        println!("Endereço IP de Destino: {}", ipv4_packet.get_destination());
        println!("Porta de Origem: {}", src_port);
        println!("Porta de Destino: {}", dst_port);
        println!("Tamanho do Payload: {} bytes", payload_size);

        // Atualiza as métricas
        metrics.add_data("UDP", payload_size);
    } else {
        println!("Erro ao processar pacote UDP");
    }
}
